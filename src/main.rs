use ctru::{
    gfx::{Gfx, Screen},
    console::Console,
    services::apt::Apt,
    services::hid::{Hid, KeyPad},
};
use ctru_sys::{
    C2D_Sprite as Sprite,
    C2D_SpriteSheet as SpriteSheet,
    C2D_DEFAULT_MAX_OBJECTS,
    C3D_RenderTarget,
    C3D_DEFAULT_CMDBUF_SIZE,
    GFX_TOP,
    GFX_LEFT,
};
use libc::size_t;

fn main() {
    let apt = Apt::init().unwrap();
    let hid = Hid::init().unwrap();
    let gfx = Gfx::default();
    let console = Console::init(Screen::Bottom);
    console.select();
    let screen: *mut C3D_RenderTarget;
    let sprite_sheet: SpriteSheet;
    let mut sprite: Sprite;
    unsafe {
        ctru_sys::C3D_Init(C3D_DEFAULT_CMDBUF_SIZE);
        ctru_sys::C2D_Init(C2D_DEFAULT_MAX_OBJECTS);
        ctru_sys::C2D_Prepare();
        screen = ctru_sys::C2D_CreateScreenTarget(GFX_TOP, GFX_LEFT);
        sprite_sheet = ctru_sys::C2D_SpriteSheetLoad("romfs:/barista.t3x\0".as_ptr());
        if sprite_sheet.is_null() {
            panic!("Sprite sheet barista.t3x not found");
        }
        sprite = citro2d::sprite_from_sheet(sprite_sheet, 0);
    }

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