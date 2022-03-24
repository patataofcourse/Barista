// actual good wrappers coming whenever i feel like it

use ctru_sys::{C2D_Sprite, C2D_SpriteSheet, C2D_SpriteSheetGetImage, C2D_DrawParams, C2D_DrawParams__bindgen_ty_1 as C2D_DrawParams_pos, C2D_DrawParams__bindgen_ty_2 as C2D_DrawParams_center};

pub unsafe fn sprite_from_sheet(sheet: C2D_SpriteSheet, index: u32) -> C2D_Sprite {
    let image = C2D_SpriteSheetGetImage(sheet, index);
    C2D_Sprite {
        image,
        params: C2D_DrawParams {
            pos: C2D_DrawParams_pos {
                x: 0.0,
                y: 0.0,
                w: (*image.subtex).width as f32,
                h: (*image.subtex).height as f32,
            },
            center: C2D_DrawParams_center {
                x: 0.0,
                y: 0.0,
            },
            angle: 0.0,
            depth: 0.0,
        }
    }
}