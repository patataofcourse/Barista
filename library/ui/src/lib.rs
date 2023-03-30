use citro2d_sys::{
    C2D_Flush, C2D_TargetClear, C3D_FrameBegin, C3D_RenderTarget, C2D_DEFAULT_MAX_OBJECTS,
    C3D_DEFAULT_CMDBUF_SIZE, C3D_FRAME_SYNCDRAW, GFX_LEFT,
};

#[macro_use]
extern crate mopa;

pub mod sprite;
pub mod text;

pub use text::Text;

#[repr(u32)]
#[derive(Clone, Debug, Copy)]
pub enum Screen {
    Top = 0,
    Bottom = 1,
}

pub struct BaristaUI {
    pub top_scene: Option<Scene>,
    pub bottom_scene: Option<Scene>,
    top_screen_target: *mut C3D_RenderTarget,
    bottom_screen_target: *mut C3D_RenderTarget,
}

impl BaristaUI {
    pub(crate) fn get_target(&self, screen: Screen) -> *mut C3D_RenderTarget {
        match screen {
            Screen::Top => self.top_screen_target.clone(),
            Screen::Bottom => self.bottom_screen_target.clone(),
        }
    }

    pub fn init() -> Self {
        unsafe {
            citro2d_sys::C3D_Init(C3D_DEFAULT_CMDBUF_SIZE);
            citro2d_sys::C2D_Init(C2D_DEFAULT_MAX_OBJECTS.into());
            citro2d_sys::C2D_Prepare();
            BaristaUI {
                bottom_scene: None,
                top_scene: None,
                top_screen_target: citro2d_sys::C2D_CreateScreenTarget(
                    Screen::Top as u32,
                    GFX_LEFT,
                ),
                bottom_screen_target: citro2d_sys::C2D_CreateScreenTarget(
                    Screen::Bottom as u32,
                    GFX_LEFT,
                ),
            }
        }
    }

    pub fn render(&self) -> [(bool, Vec<bool>); 2] {
        unsafe {
            C3D_FrameBegin(C3D_FRAME_SYNCDRAW as u8);
            let out_top = match &self.top_scene {
                Some(c) => c.draw(),
                None => (false, vec![]),
            };
            let out_bottom = match &self.bottom_scene {
                Some(c) => c.draw(),
                None => (false, vec![]),
            };
            citro2d_sys::C3D_FrameEnd(0);
            [out_top, out_bottom]
        }
    }

    pub fn set_scene(&mut self, screen: Screen, scene: impl Fn(&Self, Screen) -> Scene) {
        match screen {
            Screen::Top => self.top_scene = Some(scene(self, screen)),
            Screen::Bottom => self.bottom_scene = Some(scene(self, screen)),
        }
    }

    pub fn get_scene(&self, screen: Screen) -> Option<&Scene> {
        match screen {
            Screen::Top => self.top_scene.as_ref(),
            Screen::Bottom => self.bottom_scene.as_ref(),
        }
    }

    pub fn get_scene_mut(&mut self, screen: Screen) -> Option<&mut Scene> {
        match screen {
            Screen::Top => self.top_scene.as_mut(),
            Screen::Bottom => self.bottom_scene.as_mut(),
        }
    }

    pub fn get_object(&self, screen: Screen, name: &str) -> Option<&Box<dyn Object>> {
        match screen {
            Screen::Top => self.top_scene.as_ref()?.get_object(name),
            Screen::Bottom => self.bottom_scene.as_ref()?.get_object(name),
        }
    }

    pub fn get_object_mut(&mut self, screen: Screen, name: &str) -> Option<&mut Box<dyn Object>> {
        match screen {
            Screen::Top => self.top_scene.as_mut()?.get_object_mut(name),
            Screen::Bottom => self.bottom_scene.as_mut()?.get_object_mut(name),
        }
    }

    pub fn downcast_object<T: Object>(&self, screen: Screen, name: &str) -> Option<&T> {
        match screen {
            Screen::Top => self
                .top_scene
                .as_ref()?
                .get_object(name)
                .unwrap()
                .downcast_ref(),
            Screen::Bottom => self
                .bottom_scene
                .as_ref()?
                .get_object(name)
                .unwrap()
                .downcast_ref(),
        }
    }

    pub fn downcast_object_mut<T: Object>(&mut self, screen: Screen, name: &str) -> Option<&mut T> {
        match screen {
            Screen::Top => self
                .top_scene
                .as_mut()?
                .get_object_mut(name)
                .unwrap()
                .downcast_mut(),
            Screen::Bottom => self
                .bottom_scene
                .as_mut()?
                .get_object_mut(name)
                .unwrap()
                .downcast_mut(),
        }
    }
}

pub struct Scene {
    target: *mut C3D_RenderTarget,
    screen: Screen,
    pub background: Option<sprite::Image>,
    pub objects: Vec<(&'static str, Box<dyn Object>)>,
}

impl Scene {
    pub fn new(ui: &BaristaUI, screen: Screen, background: Option<sprite::Image>) -> Self {
        Self {
            screen: screen,
            target: ui.get_target(screen),
            background,
            objects: Vec::new(),
        }
    }

    pub fn get_screen(&self) -> Screen {
        self.screen.clone()
    }

    pub fn switch_screen(&mut self, ui: &BaristaUI, screen: Screen) {
        self.screen = screen;
        self.target = ui.get_target(screen);
    }

    pub fn draw(&self) -> (bool, Vec<bool>) {
        unsafe {
            C2D_TargetClear(self.target, 0xFFFFFFFF);
            C2D_Flush();
            citro2d_sys::C3D_FrameDrawOn(self.target);
            citro2d_sys::C2D_SceneSize(
                (*self.target).frameBuf.width.into(),
                (*self.target).frameBuf.height.into(),
                true,
            );
        }
        let out1 = match &self.background {
            Some(c) => c.draw(0, 0, 1.0, 1.0, 0.0, 0.0),
            None => false,
        };
        let mut out2 = vec![];
        for (_, object) in &self.objects {
            out2.push(object.as_ref().draw());
        }
        (out1, out2)
    }
}

impl Scene {
    pub fn add_object<T>(&mut self, name: &'static str, object: T)
    where
        T: Object + 'static,
    {
        self.objects.push((name, Box::from(object)));
    }

    pub fn get_object(&self, name: &str) -> Option<&Box<dyn Object>> {
        for (oname, object) in &self.objects {
            if oname == &name {
                return Some(object);
            }
        }
        None
    }

    pub fn get_object_mut(&mut self, name: &str) -> Option<&mut Box<dyn Object>> {
        for (oname, object) in &mut self.objects {
            if oname == &name {
                return Some(object);
            }
        }
        None
    }
}

pub trait Object: mopa::Any {
    fn draw(&self) -> bool;
}

mopafy!(Object);

/// An Object with only one sprite associated
pub struct StaticObject {
    pub x: u16,
    pub y: u16,
    pub scale_x: f32,
    pub scale_y: f32,
    pub rotation: f32,
    pub depth: f32,
    pub image: sprite::Image,
}

impl Object for StaticObject {
    fn draw(&self) -> bool {
        self.image.draw(
            self.x,
            self.y,
            self.scale_x,
            self.scale_y,
            self.rotation,
            self.depth,
        )
    }
}

/*
/// An Object with multiple sprites associated, which can be switched between
pub struct MultiSpriteObj {
    pub x: u16,
    pub y: u16,
    pub scale_x: f32,
    pub scale_y: f32,
    pub rotation: f32,
    pub depth: f32,
    pub images: HashMap<String, MultiSpriteSpr>,
}

pub struct MultiSpriteSpr {
    pub x_offset: u16,
    pub y_offset: u16,
    pub image: sprite::Image,
}

*/
