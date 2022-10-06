#![feature(allocator_api)]

extern crate barista_ui as ui_lib;

use ctru::{
    console::Console,
    gfx::{Gfx, Screen},
    services::{apt::Apt, hid::Hid},
};
use std::{
    panic::{self, PanicInfo},
    process,
};
use ui_lib::BaristaUI;

mod error;
pub use self::error::{Error, Result};

#[cfg(feature = "audio")]
mod audio;

mod format;
mod launcher;
mod scene;
pub use self::scene::menu::{MenuAction, MenuState};

#[allow(warnings)]
pub(crate) mod plgldr;

use launcher::GameVer;

static mut CONFIG: Option<format::saltwater_cfg::Config> = None;

#[cfg(feature = "audio")]
static mut AUDIO: Option<*const audio::AudioManager> = None;

fn main() {
    let apt = Apt::init().unwrap();
    let hid = Hid::init().unwrap();
    let gfx = Gfx::default();
    let console = Console::init(Screen::Bottom);
    unsafe {
        ctru_sys::romfsMountSelf("romfs\0".as_ptr());
        ctru_sys::ndspInit();
        ctru_sys::ndspSetOutputMode(ctru_sys::NDSP_OUTPUT_STEREO);
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

    #[allow(unused)]
    let mut audio_player;

    #[allow(unused)]
    #[cfg(not(feature = "audio"))]
    {
        audio_player = ();
    }

    #[cfg(feature = "audio")]
    {
        // Music test
        audio_player = audio::AudioManager::new();

        // Initial values for audio player
        audio_player.load("romfs:/audio/strm/bartender_construction.bcstm".to_string());
        audio_player.play();

        unsafe { AUDIO = Some(&audio_player) }
    }

    // Init config
    *config_wrapped() =
        Some(format::saltwater_cfg::Config::from_file("/spicerack/bin/saltwater.cfg").unwrap());

    // Main loop
    while apt.main_loop() {
        gfx.wait_for_vblank();

        hid.scan_input();

        ui.render();

        menu.run(&hid, &console, &versions);

        match &menu.action {
            MenuAction::Exit => break,
            MenuAction::Run => {
                game_to_load = Some(versions[menu.cursor as usize].clone());
                break;
            }
            #[cfg(feature = "audio")]
            MenuAction::ToggleAudio => {
                if audio_player.is_playing() {
                    audio_player.pause()
                } else {
                    audio_player.play()
                }
            }
            MenuAction::ChangeMenu(_) | MenuAction::None | MenuAction::MoveCursor => {}
        }
    }

    unsafe {
        ctru_sys::romfsUnmount("romfs\0".as_ptr());
        ctru_sys::ndspExit();
    }

    drop(gfx);
    drop(hid);
    drop(console);

    if let Some(c) = game_to_load {
        launcher::launch(c)
    }
}

fn config() -> &'static mut format::saltwater_cfg::Config {
    unsafe { CONFIG.as_mut().expect("Config not initialized") }
}

fn config_wrapped() -> &'static mut Option<format::saltwater_cfg::Config> {
    unsafe { &mut CONFIG }
}

#[cfg(feature = "audio")]
fn audio<'a>() -> &'a audio::AudioManager {
    unsafe { &*AUDIO.expect("Audio not initialized") }
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
            aptExit, errorConf, errorDisp, errorInit, errorText, CFG_LANGUAGE_EN,
            ERROR_TEXT_WORD_WRAP,
        };
        let mut error_conf: errorConf = errorConf::default();
        errorInit(&mut error_conf, ERROR_TEXT_WORD_WRAP, CFG_LANGUAGE_EN);
        errorText(&mut error_conf, msg.as_ptr() as *const ::libc::c_char);

        // Display the error
        errorDisp(&mut error_conf);
        aptExit();
    }

    process::exit(1);
}
