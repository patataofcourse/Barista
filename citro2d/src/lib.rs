// actual good wrappers coming whenever i feel like it

use ctru_sys::{C2D_Sprite, C2D_SpriteSheet, C2D_SpriteSheetGetImage, C2D_DrawParams, C2D_DrawParams__bindgen_ty_1 as C2D_DrawParams_pos, C2D_DrawParams__bindgen_ty_2 as C2D_DrawParams_center};
use ctru_sys::{C3D_DEFAULT_CMDBUF_SIZE, C2D_DEFAULT_MAX_OBJECTS, C3D_RenderTarget};

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
                x: (*image.subtex).width as f32 / 2.0,
                y: (*image.subtex).height as f32 / 2.0,
            },
            angle: 0.0,
            depth: 0.0,
        }
    }
}

pub fn init(cmdbuf_size: Option<u32>, max_objects: Option<u32>) {
    //TODO: check when it starts crahing
    unsafe {
        ctru_sys::C3D_Init(cmdbuf_size.unwrap_or(C3D_DEFAULT_CMDBUF_SIZE));
        ctru_sys::C2D_Init(max_objects.unwrap_or(C2D_DEFAULT_MAX_OBJECTS));
        ctru_sys::C2D_Prepare();
    }
}

pub fn color(r: u8, g: u8, b: u8, a: u8) -> u32{
    r as u32 + (g as u32) << 8 + (b as u32) << 16 + (a as u32) << 24
}

pub const BLACK: u32 = 0x000000FF;
pub const WHITE: u32 = 0xFFFFFFFF;

pub unsafe fn scene_begin(target: *mut C3D_RenderTarget) {
    ctru_sys::C2D_Flush();
	ctru_sys::C3D_FrameDrawOn(target);
	scene_target(target);
}

pub unsafe fn scene_target(target: *mut C3D_RenderTarget) {
    ctru_sys::C2D_SceneSize((*target).frameBuf.width.into(), (*target).frameBuf.height.into(), (*target).linked);
}