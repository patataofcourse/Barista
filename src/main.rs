extern crate barista_ui as ui_lib;

use ctru::{
    gfx::{Gfx, Screen, Side},
    console::Console,
    services::apt::Apt,
    services::hid::{Hid, KeyPad},
};
use ctru_sys::{
    C3D_RenderTarget,
};
use ui_lib::{SpriteSheet, BaristaUI};

mod launcher;
#[allow(warnings)]
pub(crate) mod plgldr;

use launcher::GameVer;

fn main() {
    let apt = Apt::init().unwrap();
    let hid = Hid::init().unwrap();
    let gfx = Gfx::default();
    let console = Console::init(Screen::Bottom);
    unsafe {
        ctru_sys::romfsMountSelf("romfs\0".as_ptr());
    }
    let screen: *mut C3D_RenderTarget;
    let ui = BaristaUI::init();
    unsafe {
        screen = ctru_sys::C2D_CreateScreenTarget(Screen::Top as u32, Side::Left as u32);
    }
    let bg_sheet = SpriteSheet::from_file("romfs:/gfx/bg.t3x").expect("No spritesheet bg.t3x!");
    let bg = bg_sheet.get_sprite(0).unwrap();
    let fg = bg_sheet.get_sprite(1).unwrap();

    let barista_sheet = SpriteSheet::from_file("romfs:/gfx/barista.t3x").expect("No spritesheet barista.t3x!");
    let barista = barista_sheet.get_sprite(0).unwrap();
    let nicole = barista_sheet.get_sprite(1).unwrap();

    let sign_sheet = SpriteSheet::from_file("romfs:/gfx/sign.t3x").expect("No spritesheet barista.t3x!");
    let sign = sign_sheet.get_sprite(0).unwrap();
    let sign_text = sign_sheet.get_sprite(1).unwrap();

    let versions = launcher::get_available_games();

    let mut game_to_load: Option<GameVer> = None;
    launcher::check_for_plgldr();
    
    println!("Welcome to Barista!");
    if versions.len() > 0 {
        println!(" - Press A to boot Saltwater");
        println!(" - Press D-Pad up/down to select a game version");
    } else {
        println!("No compatible versions of the game found");
    }
    println!(" - Press Start to exit");
    println!();
    for version in &versions {
        println!(" - [ ] {}", version);
    }
    print!("\x1b[5;4Hx");

    let mut chosen_version = 0;

    while apt.main_loop() {
        gfx.wait_for_vblank();

        hid.scan_input();
        if hid.keys_down().contains(KeyPad::KEY_START) {
            break;
        }

        if versions.len() != 0 {
            if hid.keys_down().contains(KeyPad::KEY_DUP) {
                if chosen_version > 0 {
                    chosen_version -= 1;
                    for i in 0..versions.len() {
                        print!("\x1b[{};4H{}", 5 + i, if chosen_version == i { "x" } else {" "})
                    }
                }
            }

            if hid.keys_down().contains(KeyPad::KEY_DDOWN) {
                if chosen_version < versions.len() - 1 {
                    chosen_version += 1;
                    for i in 0..versions.len() {
                        print!("\x1b[{};4H{}", 5 + i, if chosen_version == i { "x" } else {" "})
                    }
                }
            }

            if hid.keys_down().contains(KeyPad::KEY_A) {
                game_to_load = Some(versions[chosen_version].clone());
                break;
            }
        }

        // Render the scene
        unsafe {
            use ctru_sys::*;
            C3D_FrameBegin(C3D_FRAME_SYNCDRAW as u8);
            C2D_TargetClear(screen, 0xFFFFFFFF);
            ctru_sys::C2D_Flush();
            ctru_sys::C3D_FrameDrawOn(screen);
            ctru_sys::C2D_SceneSize(
                (*screen).frameBuf.width.into(),
                (*screen).frameBuf.height.into(),
                (*screen).linked,
            );
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

    unsafe {
        ctru_sys::romfsUnmount("romfs\0".as_ptr());
    }
    drop(gfx);
    drop(hid);
    drop(console);

    if let Some(c) = game_to_load {
        launcher::launch(c)
    }

}