#![allow(dead_code)]

#[allow(warnings)]
mod bindings;

#[repr(C)]
pub struct SaltwaterParams {
    pub barista: u16,
    pub reenable_rhmpatch: bool,
    pub disable_plgldr: bool,
    pub null: [u32; 31],
}

impl Default for SaltwaterParams {
    fn default() -> Self {
        Self {
            barista: 0xD06,
            reenable_rhmpatch: false,
            disable_plgldr: false,
            null: [0; 31],
        }
    }
}

use std::ffi::CString;

pub fn init() -> Result<(), i32> {
    let result = unsafe { bindings::plgLdrInit() };
    match result {
        0 => Ok(()),
        c => Err(c),
    }
}

pub fn exit() {
    unsafe { bindings::plgLdrExit() }
}

pub fn is_enabled() -> Result<bool, i32> {
    let mut res_ok = false;
    let result = unsafe { bindings::PLGLDR__IsPluginLoaderEnabled(&mut res_ok) };
    match result {
        0 => Ok(res_ok),
        c => Err(c),
    }
}

pub fn set_state(enabled: bool) -> Result<(), i32> {
    let result = unsafe { bindings::PLGLDR__SetPluginLoaderState(enabled) };
    match result {
        0 => Ok(()),
        c => Err(c),
    }
}

static mut PARAMS: bindings::PluginLoadParameters = bindings::PluginLoadParameters {
    noFlash: false,
    lowTitleId: 0,
    path: [0; 256],
    config: [0; 32],
};

pub fn set_params(
    no_flash: bool,
    low_title_id: u32,
    path: CString,
    config: SaltwaterParams,
) -> Result<(), i32> {
    let path = path.as_bytes();
    if path.len() > 256 {
        panic!(
            "Path to call plugin loader is too long, {} characters",
            path.len()
        )
    }
    let mut path_bytes = [0u8; 256];
    let mut c = 0;
    for byte in path {
        path_bytes[c] = *byte;
        c += 1;
    }

    let config = unsafe { std::mem::transmute(config) };

    let result = unsafe {
        PARAMS = bindings::PluginLoadParameters {
            noFlash: no_flash,
            lowTitleId: low_title_id,
            path: path_bytes,
            config,
        };
        bindings::PLGLDR__SetPluginLoadParameters(&mut PARAMS)
    };
    match result {
        0 => Ok(()),
        c => Err(c),
    }
}
