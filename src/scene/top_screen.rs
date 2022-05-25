use ctru::gfx::Screen;
use std::collections::HashMap;
use ui_lib::{BaristaUI, Image, Object, Scene, SpriteSheet, StaticObject};

pub struct BaristaSprites {
    pub depth: f32,
    pub images: HashMap<String, (u16, u16, Image)>,
    pub cur_image: &'static str,
}

impl Object for BaristaSprites {
    fn draw(&self) -> bool {
        let img = match self.images.get(self.cur_image) {
            Some(c) => c,
            None => return true,
        };
        img.2.draw(img.0, img.1, 1.0, 1.0, 0.0, self.depth)
    }
}

impl BaristaSprites {
    fn switch(&mut self, img: &'static str) {
        self.cur_image = img;
    }
}

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

    scene
}
