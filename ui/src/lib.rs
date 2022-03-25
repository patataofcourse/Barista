const PI: f32 = std::f64::consts::PI as f32;

use ctru_sys::{C2D_Sprite, C2D_SpriteSheet, C2D_SpriteSheetGetImage, C2D_DrawParams, C2D_DrawParams__bindgen_ty_1 as C2D_DrawParams_pos, C2D_DrawParams__bindgen_ty_2 as C2D_DrawParams_center};
use ctru_sys::{C3D_DEFAULT_CMDBUF_SIZE, C2D_DEFAULT_MAX_OBJECTS, C3D_RenderTarget, C2D_Image, C2D_DrawImage};

pub struct Scene {

}

pub trait Object {

}

pub struct Image(C2D_Image);

impl Image {
    pub fn draw(
        &self,
        x: u16,
        y: u16,
        scale_x: f32,
        scale_y: f32,
        rotation: f32,
        depth: f32
    ) -> bool {
        unsafe {
            let mut params = C2D_DrawParams {
                pos: C2D_DrawParams_pos {
                    x: 240.0 - y as f32,
                    y: x as f32,
                    w: (*self.0.subtex).width as f32 * scale_y,
                    h: (*self.0.subtex).height as f32 * scale_x,
                },
                center: C2D_DrawParams_center {
                    x: 0.0,
                    y: 0.0,
                },
                angle: (rotation + 90.0) / 360.0 * 2.0 * PI,
                depth: depth,
            };
            C2D_DrawImage(self.0, &mut params, std::ptr::null())
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
            val = ctru_sys::C2D_SpriteSheetLoad(filename.as_ptr());
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
            image = C2D_SpriteSheetGetImage(self.val, index);
        }
        if image.subtex.is_null() {
            None
        } else {
            Some(Image(image))
        }
    }
}