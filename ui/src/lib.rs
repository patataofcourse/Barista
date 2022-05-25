const PI: f32 = std::f64::consts::PI as f32;

use ctru::gfx::Screen;
use ctru_sys::{
    C2D_SpriteSheet,
    C2D_SpriteSheetGetImage,
    C2D_DrawParams,
    C2D_DrawParams__bindgen_ty_1 as C2D_DrawParams_pos,
    C2D_DrawParams__bindgen_ty_2 as C2D_DrawParams_center,
    C3D_DEFAULT_CMDBUF_SIZE,
    C2D_DEFAULT_MAX_OBJECTS,
    C3D_RenderTarget,
    C2D_Image,
    C2D_DrawImage,
    C2D_TargetClear,
    C2D_Flush,
    C3D_FRAME_SYNCDRAW,
    C3D_FrameBegin,
    GFX_LEFT,
    C2D_CreateScreenTarget,
};
use std::collections::HashMap;


pub struct BaristaUI<'a> {
    pub top_scene: Option<&'a Scene<'a>>,
    pub bottom_scene: Option<&'a Scene<'a>>,
    top_screen_target: *mut C3D_RenderTarget,
    bottom_screen_target: *mut C3D_RenderTarget,
}

impl<'a> BaristaUI <'a> {
    pub(crate) fn get_target(&self, screen: Screen) -> *mut C3D_RenderTarget {
        match screen {
            Screen::Top => self.top_screen_target.clone(),
            Screen::Bottom => self.bottom_screen_target.clone(),
        }
    }

    pub fn init() -> Self {
        unsafe {
            ctru_sys::C3D_Init(C3D_DEFAULT_CMDBUF_SIZE);
            ctru_sys::C2D_Init(C2D_DEFAULT_MAX_OBJECTS);
            ctru_sys::C2D_Prepare();
            BaristaUI {
                bottom_scene: None,
                top_scene: None,
                top_screen_target: ctru_sys::C2D_CreateScreenTarget(Screen::Top as u32, GFX_LEFT),
                bottom_screen_target: ctru_sys::C2D_CreateScreenTarget(Screen::Top as u32, GFX_LEFT),
            }
        }
    }

    pub fn render(&self) {
        unsafe {
            C3D_FrameBegin(C3D_FRAME_SYNCDRAW as u8);
            match &self.top_scene {
                Some(c) => {
                    c.draw()
                },
                None => (),
            }
            match &self.bottom_scene {
                Some(c) => {
                    c.draw()
                },
                None => (),
            }
            ctru_sys::C3D_FrameEnd(0);
        }
    }

    pub fn set_scene(&mut self, screen: Screen, scene: &'a Scene) {
        match screen {
            Screen::Top => self.top_scene = Some(scene),
            Screen::Bottom => self.bottom_scene = Some(scene),
        }
    }
}

pub struct Scene<'a> {
    target: *mut C3D_RenderTarget,
    screen: Screen,
    pub background: Option<Image>,
    pub objects: Vec<Box<dyn Object + 'a>>,
}

impl Scene<'_> {
    pub fn new(ui: &BaristaUI, screen: Screen, background: Option<Image>) -> Self {
        unsafe {
            Self {
                screen: screen.clone(),
                target: ui.get_target(screen),
                background,
                objects: vec![],
            }
        }
    }

    pub fn get_screen(&self) -> Screen {
        self.screen.clone()
    }

    //TODO: set_screen? doubt it'll be useful though

    pub fn draw(&self) {
        unsafe {
            C2D_TargetClear(self.target, 0xFFFFFFFF);
            C2D_Flush();
            ctru_sys::C3D_FrameDrawOn(self.target);
            ctru_sys::C2D_SceneSize(
                (*self.target).frameBuf.width.into(),
                (*self.target).frameBuf.height.into(),
                (*self.target).linked,
            );
        }
        match &self.background{
            Some(c) => c.draw(0, 0, 1.0, 1.0, 0.0, 0.0), //TODO: use this
            None => true,
        };
        for object in &self.objects {
            object.as_ref().draw(); //TODO: use these
        }
    }
}

impl<'a> Scene<'a> {
    pub fn add_object<T: 'a>(&mut self, object: T)
        where T: Object
    {
        self.objects.push(Box::from(object))
    }
}

pub trait Object {
    fn draw(&self) -> bool;
}

/// An Object with only one sprite associated
pub struct StaticObject {
    pub x: u16,
    pub y: u16,
    pub scale_x: f32,
    pub scale_y: f32,
    pub rotation: f32,
    pub depth: f32,
    pub image: Image,
}

impl Object for StaticObject {
    fn draw(&self) -> bool {
        self.image.draw(self.x, self.y, self.scale_x, self.scale_y, self.rotation, self.depth)
    }
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
            val = ctru_sys::C2D_SpriteSheetLoad((filename.to_string() + "\0").as_ptr());
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