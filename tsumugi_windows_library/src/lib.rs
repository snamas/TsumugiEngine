use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::shared::minwindef::{BOOL, TRUE, FALSE};

pub trait BoolInto {
    fn intobool(self) -> bool;
}

trait BoolFrom {
    fn from(self) -> BOOL;
}

impl BoolInto for BOOL {
    fn intobool(self) -> bool {
        match self {
            TRUE => true,
            FALSE => false,
            _ => false
        }
    }
}
pub trait wide_char{
    fn to_wide_chars(&self) -> Vec<u16>;
}

impl wide_char for str{
    fn to_wide_chars(&self) -> Vec<u16> {
        OsStr::new(self).encode_wide().chain(Some(0).into_iter()).collect()
    }
}

impl wide_char for String{
    fn to_wide_chars(&self) -> Vec<u16> {
        OsStr::new(self).encode_wide().chain(Some(0).into_iter()).collect()
    }
}


impl BoolFrom for bool {
    fn from(self) -> BOOL {
        match self {
            true => {TRUE}
            false => {FALSE}
        }
    }
}
