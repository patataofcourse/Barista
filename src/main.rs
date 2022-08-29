extern crate barista_ui as ui_lib;

use ctru::{
    console::Console,
    gfx::{Gfx, Screen},
    services::{apt::Apt, fs::Fs, hid::Hid},
};
use std::panic::{self, PanicInfo};
use ui_lib::BaristaUI;

mod error;
pub use self::error::{Error, Result};

mod launcher;
mod saltwater_cfg;
mod scene;
pub use self::scene::menu::{MenuAction, MenuState};

#[allow(warnings)]
pub(crate) mod plgldr;

use launcher::GameVer;

static mut CONFIG: Option<saltwater_cfg::Config> = None;

fn main() {
    let apt = Apt::init().unwrap();
    let fs = Fs::init().unwrap();
    let hid = Hid::init().unwrap();
    let gfx = Gfx::default();
    let console = Console::init(Screen::Bottom);
    unsafe {
        ctru_sys::romfsMountSelf("romfs\0".as_ptr());
    }

    panic::set_hook(Box::new(panic_hook));

    // Initialize GFX stuff
    let mut ui = BaristaUI::init();

    let top_scene = scene::top_screen_scene(&ui);
    ui.set_scene(Screen::Top, &top_scene);

    // Init loader
    let versions = launcher::get_available_games();

    let mut game_to_load: Option<GameVer> = None;
    launcher::check_for_plgldr();

    // Init menu
    let mut menu = MenuState::default();

    menu.render(&console, &versions);

    // Init config
    *config_wrapped() = Some(saltwater_cfg::Config::from_file("/spicerack/bin/saltwater.cfg").unwrap());

    while apt.main_loop() {
        gfx.wait_for_vblank();

        hid.scan_input();

        menu.run(&hid, &console, &versions);

        match &menu.action {
            MenuAction::Exit => break,
            MenuAction::Run => {
                game_to_load = Some(versions[menu.cursor as usize].clone());
                break;
            }
            MenuAction::ChangeMenu(_) | MenuAction::None | MenuAction::MoveCursor => {}
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

fn config() -> &'static mut saltwater_cfg::Config {
    unsafe { CONFIG.as_mut().expect("Config not initialized") }
}

fn config_wrapped() -> &'static mut Option<saltwater_cfg::Config> {
    unsafe { &mut CONFIG}
}

fn panic_hook(info: &PanicInfo) {
    let location_info = if let Some(c) = info.location() {
        format!(" at {}:{}:{}", c.file(), c.line(), c.column())
    } else {
        String::new()
    };

    let msg = if let Some(c) = info.payload().downcast_ref::<&str>() {
        format!("panic: {:?}{}\0", c, location_info)
    } else if let Some(c) = info.payload().downcast_ref::<String>() {
        format!("panic: {:?}{}\0", c, location_info)
    } else {
        format!("panic{}\0", location_info)
    };
    unsafe {
        use ctru_sys::{
            errorConf, errorDisp, errorInit, errorText, CFG_LANGUAGE_EN, ERROR_TEXT_WORD_WRAP,
        };
        let mut error_conf: errorConf = errorConf::default();
        errorInit(&mut error_conf, ERROR_TEXT_WORD_WRAP, CFG_LANGUAGE_EN);
        errorText(&mut error_conf, msg.as_ptr() as *const ::libc::c_char);

        // Display the error
        errorDisp(&mut error_conf);
    }
}
