use ctru::error::Error as CtruError;
use std::{
    fmt::{self, Display, Result as FmtResult},
    io::Error as IoError,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    CtruError(CtruError),
    IoError(IoError),
    OtherError(String),
}

impl Display for self::Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> FmtResult {
        write!(
            fmt,
            "{}",
            match self {
                Self::IoError(c) => c.to_string(),
                Self::CtruError(c) => c.to_string(),
                Self::OtherError(c) => c.to_string(),
            }
        )
    }
}

impl std::error::Error for self::Error {}

impl From<CtruError> for self::Error {
    fn from(err: CtruError) -> Self {
        Self::CtruError(err)
    }
}

impl From<IoError> for self::Error {
    fn from(err: IoError) -> Self {
        Self::IoError(err)
    }
}

pub fn error_applet(msg: String) {
    unsafe {
        use ctru_sys::{
            aptExit, errorConf, errorDisp, errorInit, errorText, CFG_LANGUAGE_EN,
            ERROR_TEXT_WORD_WRAP,
        };
        let mut error_conf: errorConf = errorConf::default();
        errorInit(&mut error_conf, ERROR_TEXT_WORD_WRAP, CFG_LANGUAGE_EN);
        errorText(&mut error_conf, msg.as_ptr() as *const ::libc::c_char);

        // Display the error
        errorDisp(&mut error_conf);
        aptExit();
    }
}
