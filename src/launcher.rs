use std::{slice, fmt::{self, Display}, ffi::CString};
use crate::plgldr;

use ctru_sys::{
    amInit,
    AM_GetTitleCount,
    AM_GetTitleList,
    amExit
};

const TITLE_JP: u64 = 0x0004000000155A00;
const TITLE_US: u64 = 0x000400000018a400;
const TITLE_EU: u64 = 0x000400000018a500;
const TITLE_KR: u64 = 0x000400000018a600;

#[derive(Debug, Clone)]
pub struct GameVer {
    pub region: GameRegion,
    pub is_digital: bool,
}

impl Display for GameVer {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{} ({})", self.region, if self.is_digital {"Digital"} else {"Physical"})
    }
}

#[derive(Debug, Clone)]
pub enum GameRegion {
    JP,
    US,
    EU,
    KR,
}

impl Display for GameRegion {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", match self {
            Self::JP => "RTTB+ (JP)".to_string(),
            Self::US => "RHM (US)".to_string(),
            Self::EU => "RPM (EU)".to_string(),
            Self::KR => "RSTB+ (KR)".to_string(),
        })
    }
}

impl GameRegion {
    pub fn id(&self) -> u32 {
        match self {
            Self::JP => TITLE_JP as u32,
            Self::US => TITLE_US as u32,
            Self::EU => TITLE_EU as u32,
            Self::KR => TITLE_KR as u32,
        }
    }
    pub fn id_long(&self) -> u64 {
        match self {
            Self::JP => TITLE_JP,
            Self::US => TITLE_US,
            Self::EU => TITLE_EU,
            Self::KR => TITLE_KR,
        }
    }
}

pub fn get_available_games() -> Vec<GameVer> {
    let mut available_games = vec![];
    unsafe {
        amInit();
        let null: *mut u32 = &mut 0;

        let sd_count: *mut u32 = &mut 0;
        AM_GetTitleCount(ctru_sys::MEDIATYPE_SD, sd_count);
        let sd_titles: *mut u64 = libc::malloc(std::mem::size_of::<u64>() * *sd_count as usize) as *mut u64;
        AM_GetTitleList(null, ctru_sys::MEDIATYPE_SD, *sd_count, sd_titles);
        let sd_slice = slice::from_raw_parts::<u64>(sd_titles, *sd_count as usize);
        for title in sd_slice {
            match title {
                &TITLE_JP =>
                    available_games.push(GameVer{region: GameRegion::JP, is_digital: true}),
                &TITLE_US =>
                    available_games.push(GameVer{region: GameRegion::US, is_digital: true}),
                &TITLE_EU =>
                    available_games.push(GameVer{region: GameRegion::EU, is_digital: true}),
                &TITLE_KR =>
                    available_games.push(GameVer{region: GameRegion::KR, is_digital: true}),
                _ => (),
            }
        }
        libc::free(sd_titles as *mut libc::c_void);
        drop(sd_titles);

        let cart_count: *mut u32 = &mut 0;
        AM_GetTitleCount(ctru_sys::MEDIATYPE_GAME_CARD, cart_count);
        let cart_titles: *mut u64 = libc::malloc(std::mem::size_of::<u64>() * *cart_count as usize) as *mut u64;
        AM_GetTitleList(null, ctru_sys::MEDIATYPE_GAME_CARD, *cart_count, cart_titles);
        let cart_slice = slice::from_raw_parts::<u64>(cart_titles, *cart_count as usize);
        for title in cart_slice {
            match title {
                &TITLE_JP =>
                    available_games.push(GameVer{region: GameRegion::JP, is_digital: false}),
                &TITLE_US =>
                    available_games.push(GameVer{region: GameRegion::US, is_digital: false}),
                &TITLE_EU => 
                    available_games.push(GameVer{region: GameRegion::EU, is_digital: false}),
                &TITLE_KR =>
                    available_games.push(GameVer{region: GameRegion::KR, is_digital: false}),
                _ => (),
            }
        }
        libc::free(cart_titles as *mut libc::c_void);
        drop(cart_titles);

        amExit();
    }
    available_games
}

pub fn check_for_plgldr() {
    let result = plgldr::init();
    plgldr::exit();
    match result {
        Ok(_) => (),
        Err(_) => panic!("Luma3DS plugin loader is not installed"), //TODO: proper error screen, install it..?
    }
}

pub fn launch(ver: GameVer) {
    plgldr::init().unwrap();
    plgldr::set_params(
        true,
        ver.region.id(),
        CString::new("/spicerack/bin/Saltwater.3gx").unwrap(),
        [0;32],
    ).unwrap();
    plgldr::exit();
    unsafe {
        ctru_sys::aptSetChainloader(
            ver.region.id_long(),
            if ver.is_digital { ctru_sys::MEDIATYPE_SD } else {ctru_sys::MEDIATYPE_GAME_CARD} as u8,
        );
    }
}