use crate::Result;
use bytestream::*;
use ctru::services::fs::{File, Fs};
use std::{
    collections::HashMap,
    io::{self, Read, Write},
    path::PathBuf, ffi::{OsStr},
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
        file.read_exact(&mut magic_buffer)?;
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
        let fs = Fs::init()?;
        let mut file = File::create(&fs.sdmc()?, file.into())?;
        file.write_all(MAGIC)?;
        for (index, string) in &self.btks {
            index.write_to(&mut file, ByteOrder::LittleEndian)?;
            (string.len() as u16).write_to(&mut file, ByteOrder::LittleEndian)?;
            for byte in string.bytes() {
                byte.write_to(&mut file, ByteOrder::LittleEndian)?;
            }
        }
        0xC000u16.write_to(&mut file, ByteOrder::LittleEndian)?;
        Ok(())
    }

    pub fn clear_deleted_mods(&mut self, mods: &[PathBuf]) {
        let mut mods_stripped = vec![];
        for r#mod in mods {
            if let Some(str) = r#mod.file_stem() {
                mods_stripped.push(str);
            }
        }

        self.btks = self.btks.clone().into_iter().filter(|(_, v)|mods_stripped.contains(&OsStr::new(&v))).collect();
    }
}
