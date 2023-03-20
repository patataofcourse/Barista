use std::{cmp::Ordering, collections::HashMap, iter::FromIterator, any::Any};
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

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
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

pub struct Textbox {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub speaker: (u16, bool),
}

impl Object for Textbox {
    fn draw(&self) -> bool {
        const OUTLINE_WIDTH: f32 = 2.0;
        const STEM_SIZE: f32 = 10.0;

        // do not change this!!
        const STEM_HEIGHT: f32 = STEM_SIZE * 2.0;

        let center_x = (self.width / 2 + self.x) as f32;

        let stem_base_x = self.speaker.0 as f32;
        let stem_base_y = if !self.speaker.1 {
            self.x
        } else {
            self.x + self.height
        } as f32;

        let (stem_point, stem_outline_l, stem_outline_r, stem_outline_px, stem_outline_py);
        match stem_base_x.total_cmp(&center_x) {
            Ordering::Equal => {
                stem_point = stem_base_x;
                stem_outline_l = OUTLINE_WIDTH / -22.5f32.cos();
                stem_outline_r = OUTLINE_WIDTH / -22.5f32.cos();
                stem_outline_px = 0.0;
                stem_outline_py = -OUTLINE_WIDTH * 2.0f32.sqrt();
            }
            Ordering::Less => {
                stem_point = stem_base_x - STEM_SIZE;
                stem_outline_l = OUTLINE_WIDTH;
                stem_outline_r = OUTLINE_WIDTH * 2.0f32.sqrt();
                stem_outline_px = OUTLINE_WIDTH;
                stem_outline_py = -OUTLINE_WIDTH * 2.0;
            }
            Ordering::Greater => {
                stem_point = stem_base_x + STEM_SIZE;
                stem_outline_l = OUTLINE_WIDTH * 2.0f32.sqrt();
                stem_outline_r = OUTLINE_WIDTH;
                stem_outline_px = -OUTLINE_WIDTH;
                stem_outline_py = -OUTLINE_WIDTH * 2.0;
            }
        };

        unsafe {
            citro2d_sys::C2D_DrawRectangle(
                self.x as f32 - (5.0 + OUTLINE_WIDTH),
                self.y as f32 - (5.0 + OUTLINE_WIDTH),
                0.0,
                self.width as f32 + 10.0 + OUTLINE_WIDTH * 2.0,
                self.height as f32 + 10.0 + OUTLINE_WIDTH * 2.0,
                0xFF000000,
                0xFF000000,
                0xFF000000,
                0xFF000000,
            );
            citro2d_sys::C2D_DrawTriangle(
                stem_base_x - STEM_SIZE,
                stem_base_y,
                0xFF000000,
                stem_base_x + STEM_SIZE,
                stem_base_y,
                0xFF000000,
                stem_point,
                stem_base_y + STEM_HEIGHT,
                0xFF000000,
                0.0,
            );
            citro2d_sys::C2D_DrawTriangle(
                stem_base_x - (STEM_SIZE - stem_outline_l),
                stem_base_y,
                0xFFFFFFFF,
                stem_base_x + (STEM_SIZE - stem_outline_r),
                stem_base_y,
                0xFFFFFFFF,
                stem_point + stem_outline_px,
                stem_base_y + STEM_HEIGHT + stem_outline_py,
                0xFFFFFFFF,
                0.0,
            );
            citro2d_sys::C2D_DrawRectangle(
                self.x as f32 - 5.0,
                self.y as f32 - 5.0,
                0.0,
                self.width as f32 + 10.0,
                self.height as f32 + 10.0,
                0xFFFFFFFF,
                0xFFFFFFFF,
                0xFFFFFFFF,
                0xFFFFFFFF,
            );
        }
        true
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
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

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Sign {
    pub fn switch(&mut self, text: &'static str) {
        self.cur_text = text;
    }
}

pub fn top_screen_scene(ui: &BaristaUI, screen: Screen) -> Scene {
    let bg_sheet = SpriteSheet::from_file("romfs:/gfx/bg.t3x").expect("No spritesheet bg.t3x!");
    let barista_sheet =
        SpriteSheet::from_file("romfs:/gfx/barista.t3x").expect("No spritesheet barista.t3x!");
    let sign_sheet =
        SpriteSheet::from_file("romfs:/gfx/sign.t3x").expect("No spritesheet sign.t3x!");

    let mut scene = Scene::new(ui, screen, Some(bg_sheet.get_sprite(0).unwrap()));

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

    // Text
    let text = ui_lib::text::Text::new(
        "Welcome! We're still under\nconstruction, sorry for the mess!".to_string(),
        20,
        20,
        18,
    );

    scene.add_object(
        "textbox",
        Textbox {
            x: 20,
            y: 20,
            width: text.width(),
            height: text.height(),
            speaker: (230, true),
        },
    );

    scene.add_object("text", text);

    scene
}

pub fn nicole_easter_egg(ui: &mut BaristaUI) {
    let top_scene = ui.get_scene_mut(Screen::Top).expect("No scene loaded in the top screen");
    let barista_sprites = top_scene.get_object_mut("barista").expect("Top screen scene is not barista::scene::top_screen");
    let barista_sprites: &mut BaristaSprites = barista_sprites.as_any_mut().downcast_mut().expect("Top screen scene is not barista::scene::top_screen");
    barista_sprites.switch("nicole");
}