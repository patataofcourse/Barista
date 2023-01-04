use std::{path::PathBuf, io::{Read, Write}};

use crate::Result;
use ctru::services::fs::{Fs, File};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct BaristaConfig {
    pub original_gates: bool,
}

impl Default for BaristaConfig {
    fn default() -> Self {
        Self {
            original_gates: false,
        }
    }
}

impl BaristaConfig {
    pub fn from_file(path: impl Into<PathBuf>) -> Result<Self> {
        let fs = Fs::init()?;
        let path = path.into();
        match File::open(&fs.sdmc()?, path.clone()) {
            Ok(mut file) => {
                let mut string = String::new();
                file.read_to_string(&mut string)?;
                toml::from_str(&string)?;
                todo!();
            }
            Err(e) => {
                let ctru::Error::Os(err) = *e.into_inner().unwrap().downcast::<ctru::Error>().unwrap() else {panic!("error not OS error")};
                if err as u32 == 0xC8804478 {
                    let config = BaristaConfig::default();
                    let mut f = File::create(&fs.sdmc()?, path)?;
                    f.write(&toml::to_string_pretty(&config)?.as_bytes())?;
                    Ok(config)
                } else {
                    Err(ctru::Error::Os(err).into())
                }
            }
        }
    }

    pub fn to_file(&self, path: impl Into<PathBuf>) -> Result<()> {
        let fs = Fs::init()?;
        let mut f = File::create(&fs.sdmc()?, path.into())?;
        f.write(&toml::to_string_pretty(self)?.as_bytes())?;
        Ok(())
    }
}