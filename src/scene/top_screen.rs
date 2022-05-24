use ctru::gfx::Screen;
use ui_lib::{BaristaUI, Object, Scene, SpriteSheet, StaticObject};

pub fn top_screen_scene<'a>(ui: &BaristaUI) -> Scene<'a> {
    let bg_sheet = SpriteSheet::from_file("romfs:/gfx/bg.t3x").expect("No spritesheet bg.t3x!");
    let barista_sheet =
        SpriteSheet::from_file("romfs:/gfx/barista.t3x").expect("No spritesheet barista.t3x!");
    let sign_sheet =
        SpriteSheet::from_file("romfs:/gfx/sign.t3x").expect("No spritesheet barista.t3x!");

    let mut scene = Scene::new(ui, Screen::Top, Some(bg_sheet.get_sprite(0).unwrap()));

    // Barista / Nicole
    let barista = barista_sheet.get_sprite(0).unwrap();
    let nicole = barista_sheet.get_sprite(1).unwrap();

    // Foreground
    scene.add_object(StaticObject {
        x: 0,
        y: 188,
        scale_x: 1.0,
        scale_y: 1.0,
        image: bg_sheet.get_sprite(1).unwrap(),
        depth: 0.0,
        rotation: 0.0,
    });

    // Sign
    let sign = sign_sheet.get_sprite(0).unwrap();
    let sign_text = sign_sheet.get_sprite(1).unwrap();

    todo!();
}
