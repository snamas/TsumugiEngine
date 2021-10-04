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
pub fn to_wide_chars(s: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    OsStr::new(s).encode_wide().chain(Some(0).into_iter()).collect()
}


impl BoolFrom for bool {
    fn from(self) -> BOOL {
        match self {
            true => {TRUE}
            false => {FALSE}
        }
    }
}
