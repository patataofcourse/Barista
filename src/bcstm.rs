use crate::{Error, Result};
use bytestream::{ByteOrder, StreamReader};
use ctru::services::fs::{File, Fs};
use ctru_sys::{
    linearAlloc, linearFree, ndspAdpcmData, ndspChnSetPaused, ndspChnWaveBufClear, ndspWaveBuf,
};
use std::{
    alloc::{AllocError, Allocator, Layout},
    io::{self, Read, Seek, SeekFrom},
    path::PathBuf,
    ptr::NonNull,
};

pub struct LinearAllocator;

unsafe impl Allocator for LinearAllocator {
    fn allocate(&self, layout: Layout) -> std::result::Result<NonNull<[u8]>, AllocError> {
        unsafe {
			let out = linearAlloc(layout.size() as u32);
		}
		todo!();
    }
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        linearFree(ptr.as_ptr() as *mut libc::c_void);
    }
}

pub struct BCSTMFile {
    file: File,

    total_time: u32,
    current_time: u32,

    endian: ByteOrder,
    is_paused: bool,

    looping: bool,
    channel_count: usize,
    sample_rate: u32,

    block_loop_start: u32,
    block_loop_end: u32,
    block_count: u32,
    block_size: u32,
    block_sample_count: u32,
    last_block_size: u32,
    last_block_sample_count: u32,
    adpcm_coefs: [[u16; 16]; 2],

    current_block: u32,
    info_offset: u32,
    data_offset: u32,

    channel: [u16; 2],
    wave_buf: [[ndspWaveBuf; BCSTMFile::BUFFER_COUNT]; 2],
    adpcm_data: [[ndspAdpcmData; 2]; 2],
    buffer_data: [[Vec<u8, LinearAllocator>; BCSTMFile::BUFFER_COUNT]; 2],
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
        let fs = Fs::init()?;
        let mut file = File::open(&fs.romfs()?, filename.into())?;

        let mut magic_buf = [0u8; 4];
        file.read(&mut magic_buf)?;
        if magic_buf != [b'C', b'S', b'T', b'M'] {
            Err(Error::OtherError(format!("Not a BCSTM file")))?;
        }

        let endian = match u16::read_from(&mut file, ByteOrder::LittleEndian)? {
            0xFFFE => ByteOrder::BigEndian,
            0xFEFF => ByteOrder::LittleEndian,
            _ => Err(Error::OtherError(format!("Invalid BOM")))?,
        };

        file.seek(SeekFrom::Start(0x10));
        let section_block_count = u16::read_from(&mut file, endian);
        u16::read_from(&mut file, endian);

        todo!();
    }
    pub fn tick(&mut self) {
        self.stream_data();
    }
    pub fn play(&mut self) {
        todo!();
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

    // public inline functions
    pub fn GetTotal(&self) -> f32 {
        self.total_time as f32
    }
    pub fn GetCurrent(&self) -> f32 {
        self.current_time as f32
    }

    // protected functions
    fn stream_data(&mut self) {
        todo!();
    }
    fn fill_buffers(&mut self) {
        todo!();
    }
}

impl Read for BCSTMFile {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.file.read(buf)
    }
}

impl Seek for BCSTMFile {
    fn seek(&mut self, seek: SeekFrom) -> io::Result<u64> {
        self.file.seek(seek)
    }
}

#[repr(u16)]
enum RefType {
    ByteTable = 0x0100,
    ReferenceTable = 0x0101,
    SampleData = 0x1F00,
    DSPADPCMInfo = 0x0300,
    InfoBlock = 0x4000,
    SeekBlock = 0x4001,
    DataBlock = 0x4002,
    StreamInfo = 0x4100,
    TrackInfo = 0x4101,
    ChannelInfo = 0x4102,
}
