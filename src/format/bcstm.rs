use crate::{Error, Result};
use bytestream::{ByteOrder, StreamReader};
use ctru_sys::{
    linearAlloc, linearFree, ndspAdpcmData, ndspChnSetAdpcmCoefs, ndspChnSetFormat, ndspChnSetMix,
    ndspChnSetPaused, ndspChnSetRate, ndspChnWaveBufAdd, ndspChnWaveBufClear, ndspWaveBuf,
    DSP_FlushDataCache, NDSP_FORMAT_ADPCM, NDSP_WBUF_DONE,
};
use std::{
    alloc::{AllocError, Allocator, Layout},
    fs::File,
    io::{Read, Seek, SeekFrom},
    mem::{self, MaybeUninit},
    path::PathBuf,
    ptr::NonNull,
    slice,
};

static mut ACTIVE_NDSP_CHANNELS: u32 = 0;

#[derive(Clone)]
pub struct LinearAllocator;

unsafe impl Allocator for LinearAllocator {
    fn allocate(&self, layout: Layout) -> std::result::Result<NonNull<[u8]>, AllocError> {
        let out = unsafe { linearAlloc(layout.size() as u32) };
        match unsafe { (out as *mut u8).as_ref() } {
            Some(_) => unsafe {
                Ok(slice::from_raw_parts_mut(out as *mut u8, layout.size()).into())
            },
            None => Err(AllocError),
        }
    }
    unsafe fn deallocate(&self, ptr: NonNull<u8>, _: Layout) {
        linearFree(ptr.as_ptr() as *mut libc::c_void);
    }
}

pub struct BCSTMFile {
    file: File,

    pub is_paused: bool,

    pub looping: bool,
    pub channel_count: usize,
    pub sample_rate: u32,

    block_loop_start: u32,
    block_loop_end: u32,
    block_count: u32,
    block_size: u32,
    block_sample_count: u32,
    last_block_size: u32,
    last_block_sample_count: u32,
    adpcm_coefs: [[u16; 16]; 2],

    current_block: u32,
    data_offset: u32,

    channel: [u16; 2],
    wave_buf: [[ndspWaveBuf; Self::BUFFER_COUNT]; 2],
    adpcm_data: [[ndspAdpcmData; 2]; 2],
    buffer_data: [[Vec<u8, LinearAllocator>; Self::BUFFER_COUNT]; 2],
}

impl Drop for BCSTMFile {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.channel_count {
                ndspChnWaveBufClear(self.channel[i] as i32);
            }
        }
    }
}

impl BCSTMFile {
    pub const BUFFER_COUNT: usize = 20;

    // public functions
    pub fn open_from_file(filename: impl Into<PathBuf>) -> Result<Self> {
        let mut file = File::open(filename.into())?;

        let mut magic_buf = [0u8; 4];
        file.read(&mut magic_buf)?;
        if magic_buf != [b'C', b'S', b'T', b'M'] {
            Err(Error::OtherError(format!("BCSTM - Not a BCSTM file")))?;
        }

        let endian = match u16::read_from(&mut file, ByteOrder::LittleEndian)? {
            0xFFFE => ByteOrder::BigEndian,
            0xFEFF => ByteOrder::LittleEndian,
            _ => Err(Error::OtherError(format!("BCSTM - Invalid BOM")))?,
        };

        file.seek(SeekFrom::Start(0x10))?;
        let section_block_count = u16::read_from(&mut file, endian)?;
        u16::read_from(&mut file, endian)?;

        let mut data_offset: Option<u32> = None;
        let mut info_offset: Option<u32> = None;
        for _ in 0..section_block_count {
            let id = u16::read_from(&mut file, endian)?;
            u16::read_from(&mut file, endian)?;
            let offset = u32::read_from(&mut file, endian)?;
            u32::read_from(&mut file, endian)?; // size
            match id {
                id if id == BlockType::InfoBlock as u16 => info_offset = Some(offset),
                id if id == BlockType::DataBlock as u16 => data_offset = Some(offset),
                _ => {}
            }
        }

        let data_offset = if let Some(c) = data_offset {
            c
        } else {
            Err(Error::OtherError(
                "BCSTM: no data_offset section".to_string(),
            ))?
        };
        let info_offset = if let Some(c) = info_offset {
            c
        } else {
            Err(Error::OtherError(
                "BCSTM: no info_offset section".to_string(),
            ))?
        };

        file.seek(SeekFrom::Start(info_offset as u64 + 0x20))?;
        let encoding = u8::read_from(&mut file, endian)?;
        if encoding != 2 {
            Err(Error::OtherError(
                "BCSTM - encoding not supported (only DSP ADPCM supported)".to_string(),
            ))?
        }

        let looping = u8::read_from(&mut file, endian)? != 0;
        let channel_count = u8::read_from(&mut file, endian)? as usize;
        if channel_count > 2 {
            Err(Error::OtherError(
                "Unknown BCSTM error - channel_count".to_string(),
            ))?
        }
        u8::read_from(&mut file, endian)?;

        let sample_rate = u32::read_from(&mut file, endian)?;
        let loop_pos = u32::read_from(&mut file, endian)?;
        let loop_end = u32::read_from(&mut file, endian)?;
        let block_count = u32::read_from(&mut file, endian)?;
        let block_size = u32::read_from(&mut file, endian)?;
        let block_sample_count = u32::read_from(&mut file, endian)?;
        u32::read_from(&mut file, endian)?; // last block used bytes
        let last_block_sample_count = u32::read_from(&mut file, endian)?;
        let last_block_size = u32::read_from(&mut file, endian)?;

        let block_loop_start = loop_pos / block_sample_count;
        let block_loop_end = if loop_end % block_sample_count != 0 {
            block_count
        } else {
            loop_end / block_sample_count
        };

        while u32::read_from(&mut file, endian)? != CHANNEL_INFO {}
        {
            let offset = u32::read_from(&mut file, endian)? as i64;
            file.seek(SeekFrom::Current(offset + channel_count as i64 * 8 - 0xC))?;
        }

        // Get ADPCM data
        let mut adpcm_coefs = [[0; 16]; 2];
        let mut adpcm_data = [[ndspAdpcmData::default(); 2]; 2];
        for i in 0..channel_count {
            for j in 0..16 {
                adpcm_coefs[i][j] = u16::read_from(&mut file, endian)?;
            }
            for j in 0..1 {
                adpcm_data[i][j].index = u16::read_from(&mut file, endian)?;
                adpcm_data[i][j].history0 = i16::read_from(&mut file, endian)?;
                adpcm_data[i][j].history1 = i16::read_from(&mut file, endian)?;
            }
            u16::read_from(&mut file, endian)?;
        }

        let mut buffer_data: [MaybeUninit<Vec<u8, LinearAllocator>>; Self::BUFFER_COUNT] =
            unsafe { MaybeUninit::uninit().assume_init() };

        for elmt in &mut buffer_data[..] {
            *elmt = MaybeUninit::new(Vec::new_in(LinearAllocator));
        }

        let buffer_data = unsafe {
            mem::transmute::<_, [Vec<u8, LinearAllocator>; Self::BUFFER_COUNT]>(buffer_data)
        };

        file.seek(SeekFrom::Start(data_offset as u64 + 0x20))?;
        let mut out = Self {
            file,

            is_paused: true,

            looping,
            channel_count,
            sample_rate,

            block_loop_start,
            block_loop_end,
            block_count,
            block_size,
            block_sample_count,
            last_block_size,
            last_block_sample_count,
            adpcm_coefs,

            current_block: 0,
            data_offset,

            channel: [0, 0],
            wave_buf: [[ndspWaveBuf::default(); Self::BUFFER_COUNT]; 2],
            adpcm_data,
            buffer_data: [buffer_data.clone(), buffer_data.clone()],
        };

        unsafe {
            out.init()?;
        }
        Ok(out)
    }

    // in the original code's play function
    unsafe fn init(&mut self) -> Result<()> {
        for i in 0..self.channel_count {
            while ((ACTIVE_NDSP_CHANNELS >> self.channel[i]) & 1) == 1 {
                self.channel[i] += 1;

                if self.channel[i] >= 24 {
                    Err(Error::OtherError("No NDSP channels available".to_string()))?
                }
            }
            ACTIVE_NDSP_CHANNELS |= 1 << self.channel[i];
            ndspChnWaveBufClear(self.channel[i].into());

            let mut mix: [f32; 16] = [0.0; 16];
            if self.channel_count == 1 {
                mix[0] = 0.8;
                mix[1] = 0.8;
                mix[2] = 0.2;
                mix[3] = 0.2;
            } else if i == 0 {
                mix[0] = 0.8;
                mix[2] = 0.2;
            } else {
                mix[1] = 0.8;
                mix[3] = 0.2;
            }

            ndspChnSetMix(self.channel[i] as i32, mix.as_mut_ptr());
            ndspChnSetAdpcmCoefs(self.channel[i] as i32, self.adpcm_coefs[i].as_mut_ptr());
            ndspChnSetFormat(self.channel[i] as i32, NDSP_FORMAT_ADPCM as u16);
            ndspChnSetRate(self.channel[i] as i32, self.sample_rate as f32);

            for j in 0..Self::BUFFER_COUNT {
                self.wave_buf[i][j].status = NDSP_WBUF_DONE as u8;
            }
        }

        Ok(())
    }

    pub fn tick(&mut self) -> Result<bool> {
        unsafe { self.stream_data() }
    }

    pub fn play(&mut self) {
        if !self.is_paused {
            return;
        }
        self.is_paused = false;
        for i in 0..self.channel_count {
            unsafe {
                ndspChnSetPaused(self.channel[i] as i32, false);
            }
        }
    }

    pub fn pause(&mut self) {
        if self.is_paused {
            return;
        }
        self.is_paused = true;
        for i in 0..self.channel_count {
            unsafe {
                ndspChnSetPaused(self.channel[i] as i32, true);
            }
        }
    }

    unsafe fn stream_data(&mut self) -> Result<bool> {
        if !self.is_paused {
            for i in 0..Self::BUFFER_COUNT {
                if self.wave_buf[0][i].status != NDSP_WBUF_DONE as u8 {
                    continue;
                }
                if self.channel_count == 2 && self.wave_buf[1][i].status != NDSP_WBUF_DONE as u8 {
                    continue;
                }

                if self.current_block == self.block_loop_end {
                    if self.looping {
                        self.current_block = self.block_loop_start;
                        self.file.seek(SeekFrom::Start(
                            (self.data_offset
                                + 0x20
                                + self.block_size as u32
                                    * self.channel_count as u32
                                    * self.block_loop_start) as u64,
                        ))?;
                    } else {
                        return Ok(false);
                    }
                }

                for j in 0..self.channel_count {
                    let buf = &mut self.wave_buf[j][i];
                    *buf = ndspWaveBuf::default();

                    let block_size = if self.current_block == self.block_count - 1 {
                        self.last_block_size as usize
                    } else {
                        self.block_size as usize
                    };

                    self.buffer_data[j][i].resize(block_size, 0);
                    self.file.read(&mut self.buffer_data[j][i])?;
                    DSP_FlushDataCache(
                        self.buffer_data[j][i].as_ptr() as *const libc::c_void,
                        self.block_size,
                    );

                    if self.current_block == 0 {
                        buf.adpcm_data = &mut self.adpcm_data[j][0];
                    } else if self.current_block == self.block_loop_start {
                        buf.adpcm_data = &mut self.adpcm_data[j][1];
                    }

                    if self.current_block == self.block_count - 1 {
                        buf.nsamples = self.last_block_sample_count
                    } else {
                        buf.nsamples = self.block_sample_count
                    }

                    buf.__bindgen_anon_1.data_adpcm = self.buffer_data[j][i].as_mut_ptr();

                    ndspChnWaveBufAdd(self.channel[j] as i32, buf as *mut ndspWaveBuf);
                }
            }
            self.current_block += 1;
            Ok(true)
        } else {
            Ok(true)
        }
    }
}

const CHANNEL_INFO: u32 = 0x4102;

#[repr(u16)]
#[allow(unused)]
enum BlockType {
    InfoBlock = 0x4000,
    SeekBlock = 0x4001,
    DataBlock = 0x4002,
}
