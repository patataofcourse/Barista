use ctru::{
    gfx::{Gfx, Screen},
    console::Console,
    services::apt::Apt,
    services::hid::{Hid, KeyPad},
};
use ctru_sys::{
    C2D_Sprite,
    C2D_SpriteSheet,
    C3D_RenderTarget,
    GFX_TOP,
    GFX_LEFT,
};
use ui::{SpriteSheet, Image};

const TITLE_JP: u64 = 0x0004000000155A00;
const TITLE_US: u64 = 0x000400000018a400;
const TITLE_EU: u64 = 0x000400000018a500;
const TITLE_KR: u64 = 0x000400000018a600;

//TODO: needs refactoring
fn list_available_games() {
    unsafe {
        println!("Available versions of the game:");
        ctru_sys::amInit();
        let mut non_megamix = 0;
        let h: *mut u32 = &mut 0;

        let sd_count: *mut u32 = &mut 0;
        ctru_sys::AM_GetTitleCount(ctru_sys::MEDIATYPE_SD, sd_count);
        let sd_titles: *mut u64 = libc::malloc(std::mem::size_of::<u32>() * *sd_count as usize) as *mut u64;
        ctru_sys::AM_GetTitleList(h, ctru_sys::MEDIATYPE_SD, *sd_count, sd_titles);
        let sd_slice = std::slice::from_raw_parts::<u64>(sd_titles, *sd_count as usize);
        for title in sd_slice {
            match title {
                &TITLE_JP => println!("  - RTTB+ (JP) (Digital)"),
                &TITLE_US => println!("  - RHM (US) (Digital)"),
                &TITLE_EU => println!("  - RPM (EU) (Digital)"),
                &TITLE_KR => println!("  - RSTB+ (KR) (Digital)"),
                _ => non_megamix += 1,
            }
        }
        libc::free(sd_titles as *mut libc::c_void);
        drop(sd_titles);

        let cart_count: *mut u32 = &mut 0;
        ctru_sys::AM_GetTitleCount(ctru_sys::MEDIATYPE_GAME_CARD, cart_count);
        let cart_titles: *mut u64 = libc::malloc(std::mem::size_of::<u32>() * *cart_count as usize) as *mut u64;
        ctru_sys::AM_GetTitleList(h, ctru_sys::MEDIATYPE_GAME_CARD, *cart_count, cart_titles);
        let cart_slice = std::slice::from_raw_parts::<u64>(cart_titles, *cart_count as usize);
        for title in cart_slice {
            match title {
                &TITLE_JP => println!("  - RTTB+ (JP) (Physical)"),
                &TITLE_US => println!("  - RHM (US) (Physical)"),   // Nice joke
                &TITLE_EU => println!("  - RPM (EU) (Physical)"),
                &TITLE_KR => println!("  - RSTB+ (KR) (Physical)"),
                _ => non_megamix += 1,
            }
        }
        libc::free(cart_titles as *mut libc::c_void);
        drop(cart_titles);

        ctru_sys::amExit();
        if *sd_count + *cart_count == non_megamix {println!("  none!")}
        println!();
    }
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
    citro2d::init(None, None);
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

    println!("Welcome to Barista!");
    list_available_games();
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
        //barista.draw(255, 70, 1.0, 1.0, 0.0, 0.0);
        nicole.draw(174, 17, 1.0, 1.0, 0.0, 0.0);
        fg.draw(0, 188, 1.0, 1.0, 0.0, 0.0);
        sign.draw(30, 150, 1.0, 1.0, 0.0, 0.0);
        sign_text.draw(30, 150, 1.0, 1.0, 0.0, 0.0);
        unsafe {
            ctru_sys::C3D_FrameEnd(0);
        }
    }
}