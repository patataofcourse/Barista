extern crate barista_ui as ui_lib;

use ctru::{
    console::Console,
    gfx::{Gfx, Screen, Side},
    services::apt::Apt,
    services::hid::{Hid, KeyPad},
};
use ctru_sys::C3D_RenderTarget;
use ui_lib::{BaristaUI, Scene, SpriteSheet};

mod launcher;
mod saltwater_cfg;
mod scene;

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

    // Initialize GFX stuff
    let mut ui = BaristaUI::init();

    let top_scene = scene::top_screen_scene(&ui);
    ui.set_scene(Screen::Top, &top_scene);

    // Init loader
    let versions = launcher::get_available_games();

    let mut game_to_load: Option<GameVer> = None;
    launcher::check_for_plgldr();

    println!("Welcome to Barista!");
    if versions.len() > 0 {
        println!(" - Press A to boot Saltwater");
        println!(" - Press D-Pad up/down to choose ver.");
    } else {
        println!("No compatible versions of the game found");
    }
    println!(" - Press Start to exit");
    println!();
    for version in &versions {
        println!(" - [ ] {}", version);
    }
    println!("\x1b[6;5Hx");

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
                        println!(
                            "\x1b[{};5H{}",
                            6 + i,
                            if chosen_version == i { "x" } else { " " }
                        )
                    }
                }
            }

            if hid.keys_down().contains(KeyPad::KEY_DDOWN) {
                if chosen_version < versions.len() - 1 {
                    chosen_version += 1;
                    for i in 0..versions.len() {
                        println!(
                            "\x1b[{};5H{}",
                            6 + i,
                            if chosen_version == i { "x" } else { " " }
                        )
                    }
                }
            }

            if hid.keys_down().contains(KeyPad::KEY_A) {
                game_to_load = Some(versions[chosen_version].clone());
                break;
            }
        }

        ui.render();
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
