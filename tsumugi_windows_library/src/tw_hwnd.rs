use std::io::Error;
use std::ptr::null_mut;
use std::sync::{Arc, Mutex};
use winapi::shared::minwindef::FALSE;
use winapi::shared::windef::{HWND, HWND__, RECT};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winnt::LPCWSTR;
use winapi::um::winuser::{AdjustWindowRectEx, CreateWindowExW, CW_USEDEFAULT, GetClientRect, GetFocus, ShowWindow, ShowWindowAsync, WS_OVERLAPPEDWINDOW};
use crate::{BoolInto, wide_char};
use crate::window_hander_procedure::TwWNDCLASSEXW;

pub struct TwHWND(pub &'static mut HWND__);
#[derive(Clone)]
pub struct ArcHWND(pub Arc<Mutex<TwHWND>>);
impl TwHWND {
    pub fn new(mut _wndclassexw_opt: Option<TwWNDCLASSEXW>, mut window_rc_opt: Option<RECT>) -> TwHWND {
        let mut _wndclassexw = match _wndclassexw_opt {
            Some(v) => { v }
            None => {
                TwWNDCLASSEXW::new("TsumuGraphic")
            }
        };
        let mut window_rc = match window_rc_opt {
            Some(v) => { v }
            None => {
                const WINDOW_WIDTH: u32 = 1280;
                const WINDOW_HEIGHT: u32 = 720;
                RECT { left: 0, top: 0, right: WINDOW_WIDTH as i32, bottom: WINDOW_HEIGHT as i32 }
            }
        };
        TwHWND::tw_adjust_window_rect_ex(&mut window_rc);
        match TwHWND::tw_create_window_ex_w_result(_wndclassexw, window_rc) {
            Ok(v) => return v,
            Err(v) => panic!("{},{}", v, unsafe { GetLastError() })
        }
    }
    fn tw_adjust_window_rect_ex(window_rc: &mut RECT) {
        unsafe { AdjustWindowRectEx(window_rc, WS_OVERLAPPEDWINDOW, FALSE, 0); }
    }
    fn tw_create_window_ex_w_result(_wndclassexw: TwWNDCLASSEXW, window_rc: RECT) -> Result<TwHWND, Error> {
        let handle = unsafe {
            CreateWindowExW(0, _wndclassexw.tw_register_class_ex_w() as LPCWSTR,
                            "TsumuWindow".to_wide_chars().as_ptr(),
                            //通常、WS_VISIBLEは子ウインドウを作成する際に指定します。
                            WS_OVERLAPPEDWINDOW,
                            CW_USEDEFAULT,
                            CW_USEDEFAULT,
                            window_rc.right - window_rc.left,
                            window_rc.bottom - window_rc.top,
                            null_mut(),
                            null_mut(),
                            _wndclassexw.0.hInstance,
                            null_mut()).as_mut()
        };
        match handle {
            Some(v) => Ok(TwHWND(v)),
            None => Err(Error::last_os_error())
        }
    }
    pub fn tw_show_window(&mut self, nCmdShow: i32) -> bool {
        unsafe { ShowWindow(self.0, nCmdShow).intobool() }
    }

    pub fn tw_show_window_async(&mut self, nCmdShow: i32) -> bool {
        unsafe { ShowWindowAsync(self.0, nCmdShow).intobool() }
    }
    pub fn tw_get_focus() ->HWND{
        unsafe { GetFocus() }
    }
    pub fn tw_get_client_rect(&mut self) ->RECT{
        let mut rect:RECT = RECT{
            left: 0,
            top: 0,
            right: 0,
            bottom: 0
        };
        match unsafe {GetClientRect(self.0,&mut rect)}{
            0 => {panic!("last OS error: {:?}", Error::last_os_error())},
            _ => rect
        }
    }
}
