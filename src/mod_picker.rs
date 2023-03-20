use crate::{error::Result, format::saltwater_cfg::Config};
use ctru::services::fs::{self, Fs};
use std::{collections::HashMap, ffi::OsStr, path::PathBuf};

pub const ENTRIES_PER_PAGE: usize = 13;

pub fn get_available_mods() -> Result<Vec<PathBuf>> {
    let fs = Fs::init()?;
    let mut v = vec![];
    let sdmc = fs.sdmc()?;
    let iter = match fs::read_dir(&sdmc, "/spicerack/mods") {
        Ok(c) => c,
        Err(_) => {
            fs::create_dir_all(&sdmc, "/spicerack/mods")?;
            fs::read_dir(&sdmc, "/spicerack/mods")?
        }
    };
    for f in iter {
        let f = f?;
        let path = f.path();
        if path.as_path().extension() == Some(OsStr::new("btk")) && f.metadata()?.is_file() {
            v.push(path);
        }
    }
    Ok(v)
}

pub fn show_page(paths: &Vec<PathBuf>, cfg: &Config, page: usize) -> Vec<(String, u16)> {
    let mut out = vec![];
    for path in paths
        .iter()
        .take(paths.len().min(page * ENTRIES_PER_PAGE + ENTRIES_PER_PAGE))
        .skip(page * ENTRIES_PER_PAGE)
    {
        let name = path.file_name().unwrap().to_str().unwrap().to_owned();
        let mut out_name = name.clone();
        if name.len() > 30 {
            out_name.truncate(27);
            out_name += "...";
        }

        let mut loaded = HashMap::<String, u16>::new();

        for (k, v) in &cfg.btks {
            loaded.insert(v.clone() + ".btk", *k);
        }

        let num = if let Some(c) = loaded.get(&name) {
            *c
        } else {
            u16::MAX
        };

        out.push((out_name, num));
    }
    out
}

pub fn get_mod_name(mods: &[PathBuf], page: usize, pos: usize) -> String {
    let m = &mods[page * ENTRIES_PER_PAGE + pos];
    m.with_extension("")
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
}

pub fn is_valid_slot(slot: u16) -> bool {
    slot <= 0x67 || (0x100..=0x113).contains(&slot)
}

pub fn num_pages(paths: &Vec<PathBuf>) -> usize {
    paths.len().div_ceil(ENTRIES_PER_PAGE)
}
