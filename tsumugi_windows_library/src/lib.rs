pub mod tw_hwnd;
pub mod tw_msg;
pub mod window_hander_procedure;

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::shared::minwindef::{BOOL, TRUE, FALSE};
use winapi::um::winnt::HRESULT;
use winapi::shared::winerror::{NOERROR, S_OK};

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
pub trait HRESULTinto {
    fn result(self) -> Result<i32, i32>;
}
impl HRESULTinto for i32 {
    fn result(self) -> Result<HRESULT, HRESULT> {
        match self {
            S_OK => Ok(self),
            _ => Err(self)
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
pub trait vector_Hresult{
    fn to_result(self) ->Result<HRESULT, HRESULT>;
}

impl vector_Hresult for Vec<Result<HRESULT,HRESULT>>{
    fn to_result(self) -> Result<HRESULT, HRESULT> {
        self.iter().fold(Ok(NOERROR),|acc,x|{acc.and(*x)})
    }
}