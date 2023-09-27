use std::{
    io::{Read, Write},
    path::PathBuf,
};

use crate::Result;
use ctru::services::fs::{File, Fs};
use serde::{Deserialize, Serialize};

pub fn r#true() -> bool {
    true
}

#[derive(Serialize, Deserialize)]
pub struct BaristaConfig {
    #[serde(skip, default = "bool::default")]
    pub is_new: bool,
    pub original_gates: bool,
    #[serde(default)]
    pub slot_titles: SlotTitleMode,
    #[serde(default = "r#true")]
    pub btk_loaded_msg: bool,
    #[serde(default)]
    pub extra_rows: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum SlotTitleMode {
    Megamix,
    Original,
    Internal,
    Infernal,
}

impl Default for BaristaConfig {
    fn default() -> Self {
        Self {
            is_new: true,
            original_gates: false,
            slot_titles: SlotTitleMode::Megamix,
            btk_loaded_msg: true,
            extra_rows: false,
        }
    }
}

impl Default for SlotTitleMode {
    fn default() -> Self {
        Self::Megamix
    }
}

impl BaristaConfig {
    pub fn from_file(path: impl Into<PathBuf>) -> Result<Self> {
        let mut fs = Fs::new()?;
        let path = path.into();
        match File::open(&fs.sdmc()?, path.clone()) {
            Ok(mut file) => {
                let mut string = String::new();
                file.read_to_string(&mut string)?;
                Ok(toml::from_str(&string)?)
            }
            Err(e) => {
                let ctru::Error::Os(err) =
                    *e.into_inner().unwrap().downcast::<ctru::Error>().unwrap()
                else {
                    panic!("error not OS error")
                };
                if err as u32 == 0xC8804478 {
                    //file not found, create new cfg
                    let config = BaristaConfig::default();
                    let mut f = File::create(&fs.sdmc()?, path)?;
                    f.write_all(toml::to_string_pretty(&config)?.as_bytes())?;
                    Ok(config)
                } else {
                    Err(ctru::Error::Os(err).into())
                }
            }
        }
    }

    pub fn to_file(&self, path: impl Into<PathBuf>) -> Result<()> {
        let mut fs = Fs::new()?;
        let mut f = File::create(&fs.sdmc()?, path.into())?;
        f.write_all(toml::to_string_pretty(self)?.as_bytes())?;
        Ok(())
    }
}
