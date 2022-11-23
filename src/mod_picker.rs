use crate::error::Result;
use ctru::services::fs::{self, Fs};
use std::{ffi::OsStr, path::PathBuf};

pub fn get_available_mods() -> Result<Vec<PathBuf>> {
    let fs = Fs::init()?;
    let mut v = vec![];
    let sdmc = fs.sdmc()?;
    let iter = fs::read_dir(&sdmc, "/spicerack/mods")?;
    for f in iter {
        let f = f?;
        let path = f.path();
        if path.as_path().extension() == Some(&OsStr::new("btk")) && f.metadata()?.is_file() {
            v.push(path);
        }
    }
    Ok(v)
}

pub fn show_page(paths: &Vec<PathBuf>, loaded: &Vec<PathBuf>, page: usize) -> Vec<(String, bool)> {
    todo!();
}
