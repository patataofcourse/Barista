use citro2d_sys::{
    C2D_Flush, C2D_TargetClear, C3D_FrameBegin, C3D_RenderTarget, C2D_DEFAULT_MAX_OBJECTS,
    C3D_DEFAULT_CMDBUF_SIZE, C3D_FRAME_SYNCDRAW, GFX_LEFT,
};

pub mod sprite;
pub mod text;

pub use text::Text;

#[repr(u32)]
#[derive(Clone, Debug)]
pub enum Screen {
    Top = 0,
    Bottom = 1,
}

pub struct BaristaUI<'a> {
    pub top_scene: Option<&'a Scene<'a>>,
    pub bottom_scene: Option<&'a Scene<'a>>,
    top_screen_target: *mut C3D_RenderTarget,
    bottom_screen_target: *mut C3D_RenderTarget,
}

impl<'a> BaristaUI<'a> {
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
    pub background: Option<sprite::Image>,
    pub objects: Vec<(&'static str, Box<dyn Object + 'a>)>,
}

impl Scene<'_> {
    pub fn new(ui: &BaristaUI, screen: Screen, background: Option<sprite::Image>) -> Self {
        Self {
            screen: screen.clone(),
            target: ui.get_target(screen),
            background,
            objects: Vec::new(),
        }
    }

    pub fn get_screen(&self) -> Screen {
        self.screen.clone()
    }

    //TODO: set_screen? doubt it'll be useful though

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

impl<'a> Scene<'a> {
    pub fn add_object<T: 'a>(&mut self, name: &'static str, object: T)
    where
        T: Object,
    {
        self.objects.push((name, Box::from(object)));
    }

    pub fn get_object(&self, name: &str) -> Option<&Box<dyn Object + 'a>> {
        for (oname, object) in &self.objects {
            if oname == &name {
                return Some(&object);
            }
        }
        None
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
