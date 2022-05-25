use std::{
    collections::HashMap,
    fs::File,
    ffi::CString,
    io::{self, Write},
    path::PathBuf,
};
use bytestream::*;

pub struct Config {
    pub tickflows: HashMap<u16, Vec<u8>>,
}

impl Config {
    pub fn from_file(file: PathBuf) -> io::Result<Self> {
        todo!();
    }

    pub fn to_file(&self, file: PathBuf) -> io::Result<()> {
        let mut file = File::create(file)?;
        file.write(b"SCF\x02")?;
        for (index, string) in &self.tickflows {
            index.write_to(&mut file, ByteOrder::LittleEndian)?;
            (string.len() as u16).write_to(&mut file, ByteOrder::LittleEndian)?;
            for chr in string {
                chr.write_to(&mut file, ByteOrder::LittleEndian)?;
            }
        }
        0xC000u16.write_to(&mut file, ByteOrder::LittleEndian)?;
        Ok(())
    }
}
