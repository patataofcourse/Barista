use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

use crate::Result;
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
        let path = path.into();
        match File::open(&path) {
            Ok(mut file) => {
                let mut string = String::new();
                file.read_to_string(&mut string)?;
                Ok(toml::from_str(&string)?)
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    //file not found, create new cfg
                    let config = BaristaConfig::default();
                    std::fs::create_dir_all(path.parent().unwrap())?;
                    let mut f = File::create(path)?;
                    f.write_all(toml::to_string_pretty(&config)?.as_bytes())?;
                    Ok(config)
                } else {
                    Err(e)?
                }
            }
        }
    }

    pub fn to_file(&self, path: impl Into<PathBuf>) -> Result<()> {
        let mut f = File::create(path.into())?;
        f.write_all(toml::to_string_pretty(self)?.as_bytes())?;
        Ok(())
    }
}
