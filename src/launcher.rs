use crate::plgldr::{self, SaltwaterParams};
use ctru::services::fs::{self, File, Fs};
use libc::c_void;
use std::{
    ffi::CString,
    fmt::{self, Display},
};

use ctru_sys::{
    amExit, amInit, svcExitProcess, AM_GetTitleInfo, MEDIATYPE_GAME_CARD, MEDIATYPE_SD,
};

const TITLE_JP: u64 = 0x0004000000155A00;
const TITLE_US: u64 = 0x000400000018a400;
const TITLE_EU: u64 = 0x000400000018a500;
const TITLE_KR: u64 = 0x000400000018a600;
const TITLES: [GameRegion; 4] = [
    GameRegion::JP,
    GameRegion::US,
    GameRegion::EU,
    GameRegion::KR,
];

#[derive(Debug, Clone)]
pub struct GameVer {
    pub region: GameRegion,
    pub is_digital: bool,
}

impl Display for GameVer {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "{} ({})",
            self.region,
            if self.is_digital {
                "Digital"
            } else {
                "Physical"
            }
        )
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
        write!(
            formatter,
            "{}",
            match self {
                Self::JP => "RTTB+ (JP)".to_string(),
                Self::US => "RHM (US)".to_string(),
                Self::EU => "RPM (EU)".to_string(),
                Self::KR => "RSTB+ (KR)".to_string(),
            }
        )
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
    //TODO: ctru-rs AM, whenever they add GetTitleInfo
    unsafe {
        assert!(amInit() == 0);
        let mut null = std::mem::zeroed();

        for title in &TITLES {
            let id: *mut u64 = &mut title.id_long();
            if AM_GetTitleInfo(MEDIATYPE_SD, 1, id, &mut null) == 0 {
                match title {
                    GameRegion::JP => {
                        if cfg!(feature = "jp") {
                            available_games.push(GameVer {
                                region: GameRegion::JP,
                                is_digital: true,
                            })
                        }
                    }
                    GameRegion::US => available_games.push(GameVer {
                        region: GameRegion::US,
                        is_digital: true,
                    }),
                    GameRegion::EU => available_games.push(GameVer {
                        region: GameRegion::EU,
                        is_digital: true,
                    }),
                    GameRegion::KR => available_games.push(GameVer {
                        region: GameRegion::KR,
                        is_digital: true,
                    }),
                }
            }
        }

        for title in &TITLES {
            let id: *mut u64 = &mut title.id_long();
            if AM_GetTitleInfo(MEDIATYPE_GAME_CARD, 1, id, &mut null) == 0 {
                match title {
                    GameRegion::JP => {
                        if cfg!(feature = "jp") {
                            available_games.push(GameVer {
                                region: GameRegion::JP,
                                is_digital: false,
                            })
                        }
                    }
                    GameRegion::US => available_games.push(GameVer {
                        region: GameRegion::US,
                        is_digital: false, //this will never happen
                    }),
                    GameRegion::EU => available_games.push(GameVer {
                        region: GameRegion::EU,
                        is_digital: false,
                    }),
                    GameRegion::KR => available_games.push(GameVer {
                        region: GameRegion::KR,
                        is_digital: false,
                    }),
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

pub fn check_for_rhmpatch() -> bool {
    let mut fs = Fs::new().unwrap();
    File::open(
        &fs.sdmc().unwrap(),
        "/luma/titles/000400000018A400/code.ips",
    )
    .is_ok()
}

pub fn launch(ver: GameVer, is_citra: bool) {
    plgldr::init().unwrap();
    let mut params = SaltwaterParams::default();

    // disable rhmpatch if it exists
    if check_for_rhmpatch() {
        params.reenable_rhmpatch = true;
        let mut fs = Fs::new().unwrap();
        fs::rename(
            &mut fs.sdmc().unwrap(),
            "/luma/titles/000400000018A400/code.ips",
            "/luma/titles/000400000018A400/code.old.ips",
        )
        .unwrap()
    }

    // enable plugin loader if it's not
    if !plgldr::is_enabled().unwrap() {
        params.disable_plgldr = true;
        plgldr::set_state(true).unwrap();
    }

    plgldr::set_params(
        true,
        ver.region.id(),
        CString::new("/spicerack/bin/Saltwater.3gx").unwrap(),
        params,
    )
    .unwrap();
    plgldr::exit();

    let mediatype = if ver.is_digital {
        ctru_sys::MEDIATYPE_SD
    } else {
        ctru_sys::MEDIATYPE_GAME_CARD
    } as u8;

    unsafe {
        if is_citra {
            assert!(
                ctru_sys::APT_PrepareToDoApplicationJump(0, ver.region.id_long(), mediatype) == 0
            );
            assert!(
                ctru_sys::APT_DoApplicationJump(
                    &[] as *const c_void,
                    0,
                    &[0u8; 0x20] as *const u8 as *const c_void
                ) == 0
            );
            svcExitProcess();
        } else {
            ctru_sys::aptSetChainloader(
                ver.region.id_long(),
                if ver.is_digital {
                    ctru_sys::MEDIATYPE_SD
                } else {
                    ctru_sys::MEDIATYPE_GAME_CARD
                } as u8,
            );
        }
    }
}
