use ctru::{
    gfx::{Gfx, Screen},
    console::Console,
    services::apt::Apt,
    services::hid::{Hid, KeyPad},
};
use ctru_sys::{
    C3D_RenderTarget,
    GFX_TOP,
    GFX_LEFT,
    amInit,
    AM_GetTitleCount,
    AM_GetTitleList,
    amExit
};
use ui::SpriteSheet;
use std::{slice, fmt::{self, Display}};

const TITLE_JP: u64 = 0x0004000000155A00;
const TITLE_US: u64 = 0x000400000018a400;
const TITLE_EU: u64 = 0x000400000018a500;
const TITLE_KR: u64 = 0x000400000018a600;

struct GameVer {
    pub region: GameRegion,
    pub is_digital: bool,
}

impl Display for GameVer {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{} ({})", self.region, if self.is_digital {"Digital"} else {"Physical"})
    }
}

enum GameRegion {
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

fn get_available_games() -> Vec<GameVer> {
    let mut available_games = vec![];
    unsafe {
        println!("Available versions of the game:");
        amInit();
        let h: *mut u32 = &mut 0;

        let sd_count: *mut u32 = &mut 0;
        AM_GetTitleCount(ctru_sys::MEDIATYPE_SD, sd_count);
        let sd_titles: *mut u64 = libc::malloc(std::mem::size_of::<u64>() * *sd_count as usize) as *mut u64;
        AM_GetTitleList(h, ctru_sys::MEDIATYPE_SD, *sd_count, sd_titles);
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
        AM_GetTitleList(h, ctru_sys::MEDIATYPE_GAME_CARD, *cart_count, cart_titles);
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

fn main() {
    let apt = Apt::init().unwrap();
    let hid = Hid::init().unwrap();
    let gfx = Gfx::default();
    let console = Console::init(Screen::Bottom);
    console.select();
    unsafe {
        ctru_sys::romfsMountSelf("romfs\0".as_ptr());
    }
    let screen: *mut C3D_RenderTarget;
    ui::init();
    unsafe {
        screen = ctru_sys::C2D_CreateScreenTarget(GFX_TOP, GFX_LEFT);
    }
    let bg_sheet = SpriteSheet::from_file("romfs:/bg.t3x").expect("No spritesheet bg.t3x!");
    let bg = bg_sheet.get_sprite(0).unwrap();
    let fg = bg_sheet.get_sprite(1).unwrap();

    let barista_sheet = SpriteSheet::from_file("romfs:/barista.t3x").expect("No spritesheet barista.t3x!");
    let barista = barista_sheet.get_sprite(0).unwrap();
    let nicole = barista_sheet.get_sprite(1).unwrap();

    let sign_sheet = SpriteSheet::from_file("romfs:/sign.t3x").expect("No spritesheet barista.t3x!");
    let sign = sign_sheet.get_sprite(0).unwrap();
    let sign_text = sign_sheet.get_sprite(1).unwrap();

    let versions = get_available_games();

    println!("Welcome to Barista!");
    //println!(" - Press A to boot Saltwater");
    println!(" - Press Start to exit");
    
    while apt.main_loop() {
        gfx.wait_for_vblank();

        hid.scan_input();
        if hid.keys_down().contains(KeyPad::KEY_START) {
            break;
        }

        // Render the scene
        unsafe {
            use ctru_sys::*;
            C3D_FrameBegin(C3D_FRAME_SYNCDRAW as u8);
            C2D_TargetClear(screen, citro2d::WHITE);
            citro2d::scene_begin(screen);
        }
        bg.draw(0, 0, 1.0, 1.0, 0.0, 0.0);
        barista.draw(255, 70, 1.0, 1.0, 0.0, 0.0);
        //nicole.draw(174, 17, 1.0, 1.0, 0.0, 0.0);
        fg.draw(0, 188, 1.0, 1.0, 0.0, 0.0);
        sign.draw(30, 150, 1.0, 1.0, 0.0, 0.0);
        sign_text.draw(30, 150, 1.0, 1.0, 0.0, 0.0);
        unsafe {
            ctru_sys::C3D_FrameEnd(0);
        }
    }
}