use std::{collections::HashMap, iter::FromIterator};
use ui_lib::{
    sprite::{Image, SpriteSheet},
    BaristaUI, Object, Scene, Screen, StaticObject,
};

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

pub struct Textbox{
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub speaker: (u16, u16),
}

impl Object for Textbox {
    fn draw(&self) -> bool {
        unsafe {
            citro2d_sys::C2D_DrawRectangle(self.x as f32 - 7.0, self.y as f32 - 7.0, 0.0, self.width as f32 + 14.0, self.height as f32 + 14.0, 0xFF000000, 0xFF000000, 0xFF000000, 0xFF000000);
            //citro2d_sys::C2D_DrawTriangle(235.0, 20.0 + text.lines as f32 * 30.0 * 0.6, 0xFF000000, 265.0, 20.0 + text.lines as f32 * 30.0 * 0.6, 0xFF000000, 280.0, 80.0, 0xFF000000, 0.0);
            //citro2d_sys::C2D_DrawTriangle(240.0, 20.0 + text.lines as f32 * 30.0 * 0.6, 0xFFFFFFFF, 260.0, 20.0 + text.lines as f32 * 30.0 * 0.6, 0xFFFFFFFF, 275.0, 75.0, 0xFFFFFFFF, 0.0);
            citro2d_sys::C2D_DrawRectangle(self.x as f32 - 5.0, self.y as f32 - 5.0, 0.0, self.width as f32 + 10.0, self.height as f32 + 10.0, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF);
        }
        true
    }
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
        SpriteSheet::from_file("romfs:/gfx/sign.t3x").expect("No spritesheet sign.t3x!");

    let mut scene = Scene::new(ui, Screen::Top, Some(bg_sheet.get_sprite(0).unwrap()));

    // Barista / Nicole
    scene.add_object(
        "barista",
        BaristaSprites {
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
            cur_image: "barista",
        },
    );

    // Foreground (counter)
    scene.add_object(
        "foreground",
        StaticObject {
            x: 0,
            y: 188,
            scale_x: 1.0,
            scale_y: 1.0,
            image: bg_sheet.get_sprite(1).unwrap(),
            depth: 0.0,
            rotation: 0.0,
        },
    );

    // Sign
    scene.add_object(
        "sign",
        Sign {
            x: 30,
            y: 150,
            sign_image: sign_sheet.get_sprite(0).unwrap(),
            sign_text: HashMap::from_iter(vec![(
                "opening".to_string(),
                sign_sheet.get_sprite(1).unwrap(),
            )]),
            depth: 0.0,
            cur_text: "opening",
        },
    );

    let text = ui_lib::text::Text::new(
        "Welcome! We're still under\nconstruction, sorry for the mess!".to_string(),
        20,
        20,
        18,
    );

    // Text
    let text = ui_lib::text::Text::new(
        "Welcome! We're still under\nconstruction, sorry for the mess!".to_string(),
        20,
        20,
        18,
    );

    scene.add_object("textbox", Textbox {
        x: 20, y: 20, width: text.width(), height: text.height(), speaker: (275, 75)
    });

    scene.add_object(
        "text",
        text,
    );

    scene
}
