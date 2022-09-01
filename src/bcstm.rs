use crate::{Result, Error};
use std::{path::PathBuf, io::{self, Seek, SeekFrom, Read}};
use bytestream::{ByteOrder, StreamReader};
use ctru::services::fs::{File, Fs};
use ctru_sys::{ndspAdpcmData, ndspWaveBuf, ndspChnSetPaused};

pub struct BCSTMFile {
	file: File,

    total_time: u32,
    current_time: u32,
	
	endian: ByteOrder,
	is_streaming: bool,
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
	
	active_ndsp_channels: u32, // default: 0
	
	is_loaded: bool, // default: false
	
	channel: [u16; 2],
	wave_buf: [[ndspWaveBuf; BCSTMFile::BUFFER_COUNT]; 2],
	adpcm_data: [[ndspAdpcmData; 2]; 2],
	buffer_data: [[Vec<u8, LinearAllocator<u8>>; BCSTMFile::BUFFER_COUNT]; 2],
}

impl Drop for BCSTMFile {
	fn drop(&mut self) {
		self.stop();
	}
}

impl BCSTMFile {
    pub const BUFFER_COUNT : usize = 20;

    // constructors and destructors
    pub fn new_uninit() -> Self {
		Self {
			active_ndsp_channels: 0,
			is_loaded: false,
			is_paused: false,
			is_streaming: false,
			channel_count: 0,
			// ...
		}
	}

    // public functions
    pub fn open_from_file(&mut self, filename: impl Into<PathBuf>) -> Result<()> {
		self.stop();
		let fs = Fs::init()?;
		let file = File::open(&fs.romfs()?, filename.into())?;
		let mut endian = ByteOrder::LittleEndian;

		let mut magic_buf = [0u8; 4];
		file.read(&mut magic_buf)?;
		if magic_buf != [b'C', b'S', b'T', b'M'] {
			Err(Error::OtherError(format!("Not a BCSTM file")))?;
		}
		todo!();
	}
    pub fn tick(&mut self) {
		self.stream_data();
	}
    pub fn play(&mut self) {
		todo!();
	}
    pub fn pause(&mut self) {
		if !self.is_streaming { return; }
		self.is_paused = true;
		for i in 0..self.channel_count {
			unsafe {
				ndspChnSetPaused(self.channel[i] as i32, true);
			}
		}
	}
    pub fn stop(&mut self) {
		todo!();
	}

    // public inline functions
    pub fn GetLoop(&self) -> &'static str { if self.looping {"True"} else {"False"} }
    pub fn GetLoopStart<'a>(&self) -> &'a str { let ref out = format!("{}", self.block_loop_start); out }
    pub fn GetLoopEnd<'a>(&self) -> &'a str { let ref out = format!("{}", self.block_loop_end); out }
    pub fn GetTotal(&self) -> f32 { self.total_time as f32 }
    pub fn GetCurrent(&self) -> f32 { self.current_time as f32 }

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
	ByteTable      = 0x0100,
	ReferenceTable = 0x0101,
	SampleData     = 0x1F00,
	DSPADPCMInfo   = 0x0300,
	InfoBlock      = 0x4000,
	SeekBlock      = 0x4001,
	DataBlock      = 0x4002,
	StreamInfo     = 0x4100,
	TrackInfo      = 0x4101,
	ChannelInfo    = 0x4102,
}