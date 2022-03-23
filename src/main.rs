extern crate ctru;
extern crate ctru_sys as libctru;

use ctru::gfx::{Gfx, Screen};
use ctru::console::Console;
use ctru::services::apt::Apt;
use ctru::services::hid::{Hid, KeyPad};

fn main() {
    let apt = Apt::init().unwrap();
    let hid = Hid::init().unwrap();
    let gfx = Gfx::default();
    let console = Console::init(Screen::Bottom);
    console.select();

    println!("Welcome to Barista!");
    println!("\x1b[4;8H:)");
    println!("\x1b[29;16HPress Start to exit");
    
    while apt.main_loop() {
        gfx.flush_buffers();
        gfx.swap_buffers();
        gfx.wait_for_vblank();

        hid.scan_input();
        if hid.keys_down().contains(KeyPad::KEY_START) {
            break;
        }
        if hid.keys_down().contains(KeyPad::KEY_SELECT) {
            panic!("what if i were to panic... on purpose >:)");
        }
    }
}