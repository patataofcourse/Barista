use std::{mem, ptr};

use citro2d_sys::C2D_Text;

pub struct Text {
    inner: C2D_Text,
    pub x: u16,
    pub y: u16,
    pub size: u8,
}

impl Text {
    /// if wrap_width = 0, no wrap
    pub fn new(text: String, x: u16, y: u16, size: u8) -> Self {
        let mut text_s;
        unsafe {
            let buf = citro2d_sys::C2D_TextBufNew(text.len() as u32);
            assert!(buf != ptr::null_mut());
            text_s = mem::zeroed();
            let test_res = citro2d_sys::C2D_TextParse(&mut text_s, buf, (text + "\0").as_ptr());
            assert!(test_res != ptr::null());

            citro2d_sys::C2D_TextOptimize(&text_s);
        }

        Self {
            inner: text_s,
            x,
            y,
            size,
        }
    }

    pub fn change_text(&mut self, text: String) {
        unsafe {
            citro2d_sys::C2D_TextBufClear(self.inner.buf);
            citro2d_sys::C2D_TextBufResize(self.inner.buf, text.len() as u32);
            assert!(self.inner.buf != ptr::null_mut());
            let text_res =
                citro2d_sys::C2D_TextParse(&mut self.inner, self.inner.buf, (text + "\0").as_ptr());
            assert!(text_res != ptr::null_mut());

            citro2d_sys::C2D_TextOptimize(&self.inner);
        }
    }

    pub fn width(&self) -> u16 {
        unsafe {
            let mut w = 0.0;
            let mut _h = 0.0;
            citro2d_sys::C2D_TextGetDimensions(&self.inner, self.size as f32 / 30.0, self.size as f32 / 30.0, &mut w, &mut _h);
            w as u16
        }
    }


    pub fn height(&self) -> u16 {
        unsafe {
            let mut _w = 0.0;
            let mut h = 0.0;
            citro2d_sys::C2D_TextGetDimensions(&self.inner, self.size as f32 / 30.0, self.size as f32 / 30.0, &mut _w, &mut h);
            h as u16
        }
    }

}

impl crate::Object for Text {
    fn draw(&self) -> bool {
        unsafe {
            citro2d_sys::C2D_DrawText(
                &self.inner,
                0b00000,
                self.x as f32,
                self.y as f32,
                0.0,
                self.size as f32 / 30.0,
                self.size as f32 / 30.0,
            )
        }
        true
    }
}
