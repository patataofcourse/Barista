use crate::Result;
use bytestream::*;
use ctru::services::fs::{File, Fs};
use std::{
    collections::HashMap,
    io::{self, Read, Write},
    path::PathBuf,
};

#[derive(Default)]
pub struct Config {
    pub btks: HashMap<u16, String>,
}

const MAGIC: &[u8; 4] = b"SCF\x02";

impl Config {
    pub fn from_file(file: impl Into<PathBuf>) -> Result<Self> {
        let fs = Fs::init()?;
        let mut file = File::open(&fs.sdmc()?, file.into())?;
        let mut magic_buffer = [0u8; 4];
        file.read(&mut magic_buffer)?;
        if &magic_buffer != MAGIC {
            Err(io::Error::new(io::ErrorKind::Other, "invalid file"))?;
        }
        let mut btks = HashMap::new();
        loop {
            let index = u16::read_from(&mut file, ByteOrder::LittleEndian)?;
            if index == 0xC000 {
                break;
            }
            let file_len = u16::read_from(&mut file, ByteOrder::LittleEndian)?;
            let mut fname = String::new();
            for _ in 0..file_len {
                //TODO: non-ascii
                fname.push(u8::read_from(&mut file, ByteOrder::LittleEndian)? as char);
            }
            btks.insert(index, fname);
        }
        Ok(Self { btks })
    }

    pub fn to_file(&self, file: impl Into<PathBuf>) -> Result<()> {
        //TODO: non-ASCII
        let fs = Fs::init()?;
        let mut file = File::create(&fs.sdmc()?, file.into())?;
        file.write(MAGIC)?;
        for (index, string) in &self.btks {
            index.write_to(&mut file, ByteOrder::LittleEndian)?;
            (string.len() as u16).write_to(&mut file, ByteOrder::LittleEndian)?;
            for chr in string.chars() {
                let chru8 = chr as u8;
                chru8.write_to(&mut file, ByteOrder::LittleEndian)?;
            }
        }
        0xC000u16.write_to(&mut file, ByteOrder::LittleEndian)?;
        Ok(())
    }
}
