use ctru::gfx::Screen;
use std::{collections::HashMap, iter::FromIterator};
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
            None => return false,
        };
        img.2.draw(img.0, img.1, 1.0, 1.0, 0.0, self.depth)
    }
}

impl BaristaSprites {
    pub fn switch(&mut self, img: &'static str) {
        self.cur_image = img;
    }
}

pub struct Sign {
    pub x: u16,
    pub y: u16,
    pub sign_image: Image,
    pub sign_text: HashMap<String, Image>,
    pub cur_text: &'static str,
    pub depth: f32,
}

impl Object for Sign {
    fn draw(&self) -> bool {
        let text = match self.sign_text.get(self.cur_text) {
            Some(c) => c,
            None => return false,
        };

        self.sign_image
            .draw(self.x, self.y, 1.0, 1.0, 0.0, self.depth)
            && text.draw(self.x, self.y, 1.0, 1.0, 0.0, self.depth)
    }
}

impl Sign {
    pub fn switch(&mut self, text: &'static str) {
        self.cur_text = text;
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
    scene.add_object(BaristaSprites {
        depth: 0.0,
        images: HashMap::from_iter(vec![
            (
                "barista".to_string(),
                (255, 70, barista_sheet.get_sprite(0).unwrap()),
            ),
            (
                "nicole".to_string(),
                (174, 17, barista_sheet.get_sprite(1).unwrap()),
            ),
        ]),
        cur_image: "nicole",
    });

    // Foreground (counter)
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
    scene.add_object(Sign {
        x: 30,
        y: 150,
        sign_image: sign_sheet.get_sprite(0).unwrap(),
        sign_text: HashMap::from_iter(vec![(
            "opening".to_string(),
            sign_sheet.get_sprite(1).unwrap(),
        )]),
        depth: 0.0,
        cur_text: "opening",
    });

    scene
}
