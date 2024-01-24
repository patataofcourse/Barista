use std::sync::Mutex;

use lazy_static::lazy_static;

#[cfg(debug_assertions)]
#[allow(unused)]
pub enum Log {
    General,
    Audio,
}

#[cfg(debug_assertions)]
impl ToString for Log {
    fn to_string(&self) -> String {
        use Log::*;
        match self {
            General => "general",
            Audio => "audio",
        }
        .to_string()
    }
}

#[cfg(debug_assertions)]
lazy_static! {
    pub static ref LOG: Mutex<String> = Mutex::new(String::new());
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! log {
    ($type:ident, $lit:literal $(, $i:expr)* $(,)?) => {
        let out = {
            use $crate::log::Log::*;
            format!("<{}> {}\n", $type.to_string(), format!($lit, $($i,)?))
        };

        let mut log = $crate::log::LOG.lock().unwrap();
        *log += &out;
    };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! log {
    ($type:ident, $lit:literal $(, $i:expr)* $(,)?) => {};
}
