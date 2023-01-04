use std::f32::consts::PI;

use citro2d_sys::{
    C2D_DrawParams, C2D_DrawParams__bindgen_ty_1 as C2D_DrawParams_pos,
    C2D_DrawParams__bindgen_ty_2 as C2D_DrawParams_center, C2D_Image, C2D_SpriteSheet,
};

pub struct Image(C2D_Image);

impl Image {
    pub fn draw(
        &self,
        x: u16,
        y: u16,
        scale_x: f32,
        scale_y: f32,
        rotation: f32,
        depth: f32,
    ) -> bool {
        unsafe {
            let mut params = C2D_DrawParams {
                pos: C2D_DrawParams_pos {
                    x: x as f32,
                    y: y as f32,
                    w: (*self.0.subtex).width as f32 * scale_x,
                    h: (*self.0.subtex).height as f32 * scale_y,
                },
                center: C2D_DrawParams_center { x: 0.0, y: 0.0 },
                angle: rotation / 360.0 * 2.0 * PI,
                depth: depth,
            };
            citro2d_sys::C2D_DrawImage(self.0, &mut params, std::ptr::null())
        }
    }
}

pub struct SpriteSheet {
    pub filename: String,
    val: C2D_SpriteSheet,
}

impl SpriteSheet {
    pub fn from_file(filename: &'static str) -> Option<Self> {
        let val: C2D_SpriteSheet;
        unsafe {
            val = citro2d_sys::C2D_SpriteSheetLoad((filename.to_string() + "\0").as_ptr());
        }
        if val.is_null() {
            None
        } else {
            Some(Self {
                filename: filename.to_string(),
                val,
            })
        }
    }

    pub fn get_sprite(&self, index: u32) -> Option<Image> {
        let image: C2D_Image;
        unsafe {
            image = citro2d_sys::C2D_SpriteSheetGetImage(self.val, index);
        }
        if image.subtex.is_null() {
            None
        } else {
            Some(Image(image))
        }
    }
}
