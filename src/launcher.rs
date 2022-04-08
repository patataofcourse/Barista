use std::{slice, fmt::{self, Display}, ffi::CString};
use crate::plgldr;

use ctru_sys::{
    amInit,
    AM_GetTitleCount,
    AM_GetTitleInfo,
    AM_TitleEntry,
    amExit,
    MEDIATYPE_GAME_CARD,
    MEDIATYPE_SD
};

const TITLE_JP: u64 = 0x0004000000155A00;
const TITLE_US: u64 = 0x000400000018a400;
const TITLE_EU: u64 = 0x000400000018a500;
const TITLE_KR: u64 = 0x000400000018a600;
const TITLES: [GameRegion; 4] = [GameRegion::JP, GameRegion::US, GameRegion::EU, GameRegion::KR];

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
        let null: *mut AM_TitleEntry = &mut AM_TitleEntry {
            titleID: 0,
            size: 0,
            version: 0,
            unk: [0;6],
        };

        for title in &TITLES {
            let id: *mut u64 = &mut title.id_long();
            if AM_GetTitleInfo(MEDIATYPE_SD, 1, id, null) == 0 {
                match title {
                    GameRegion::JP =>
                        available_games.push(GameVer{region: GameRegion::JP, is_digital: true}),
                    GameRegion::US =>
                        available_games.push(GameVer{region: GameRegion::US, is_digital: true}),
                    GameRegion::EU =>
                        available_games.push(GameVer{region: GameRegion::EU, is_digital: true}),
                    GameRegion::KR =>
                        available_games.push(GameVer{region: GameRegion::KR, is_digital: true}),
                }
            }
        }

        for title in &TITLES {
            let id: *mut u64 = &mut title.id_long();
            if AM_GetTitleInfo(MEDIATYPE_GAME_CARD, 1, id, null) == 0 {
                match title {
                    GameRegion::JP =>
                        available_games.push(GameVer{region: GameRegion::JP, is_digital: false}),
                    GameRegion::US =>
                        available_games.push(GameVer{region: GameRegion::US, is_digital: false}),
                    GameRegion::EU =>
                        available_games.push(GameVer{region: GameRegion::EU, is_digital: false}),
                    GameRegion::KR =>
                        available_games.push(GameVer{region: GameRegion::KR, is_digital: false}),
                }
            }
        }
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