#![allow(dead_code)]

use std::{ffi::CString};
use static_assertions::const_assert;

use crate::format::barista_cfg::BaristaConfig;

#[allow(warnings)]
mod bindings;

#[repr(C)]
pub struct SaltwaterParams {
    pub barista: u16,
    pub reenable_rhmpatch: bool,
    pub disable_plgldr: bool,
    pub loaded_msg: bool,
    pub extra_rows: bool,
    pub null: [u8; 0x7A],
}

const_assert!(std::mem::size_of::<SaltwaterParams>() == 0x80);

impl Default for SaltwaterParams {
    fn default() -> Self {
        Self {
            barista: 0xD06,
            reenable_rhmpatch: false,
            disable_plgldr: false,
            loaded_msg: true,
            extra_rows: false,
            null: [0;0x7A],
        }
    }
}

impl SaltwaterParams {
    pub fn apply_settings(&mut self, settings: &BaristaConfig) {
        self.loaded_msg = settings.btk_loaded_msg;
        self.extra_rows = settings.extra_rows;
    }
}

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
    for (c, byte) in path.iter().enumerate() {
        path_bytes[c] = *byte;
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
