use ctru::{
    gfx::{Gfx, Screen},
    console::Console,
    services::apt::Apt,
    services::hid::{Hid, KeyPad},
};
use ctru_sys::{
    C2D_Sprite as Sprite,
    C2D_SpriteSheet as SpriteSheet,
    C3D_RenderTarget,
    GFX_TOP,
    GFX_LEFT,
};

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
    let sprite_sheet: SpriteSheet;
    let mut bg: Sprite;
    let mut fg: Sprite;
    citro2d::init(None, None);
    unsafe {
        screen = ctru_sys::C2D_CreateScreenTarget(GFX_TOP, GFX_LEFT);
        sprite_sheet = ctru_sys::C2D_SpriteSheetLoad("romfs:/bg.t3x\0".as_ptr());
        if sprite_sheet.is_null() {
            panic!("Sprite sheet bg.t3x not found");
        }
        bg = citro2d::sprite_from_sheet(sprite_sheet, 0);
        fg = citro2d::sprite_from_sheet(sprite_sheet, 1);
    }
    bg.params.pos.x = 240.0;
    bg.params.angle = std::f64::consts::PI as f32 / 2.0;
    fg.params.pos.x = 240.0 - 188.0;
    fg.params.angle = std::f64::consts::PI as f32 / 2.0;

    println!("Welcome to Barista!");
    println!("\x1b[29;12HPress Start to exit");
    
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
            C2D_DrawImage(bg.image, &mut bg.params, std::ptr::null());
            C2D_DrawImage(fg.image, &mut fg.params, std::ptr::null());
            C3D_FrameEnd(0);
        }
    }
}