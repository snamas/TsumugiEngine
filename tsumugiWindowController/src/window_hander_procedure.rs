use winapi::shared::windef::{HWND, RECT, HWND__, HICON, POINT};
use winapi::shared::minwindef::{UINT, WPARAM, LPARAM, LRESULT, HINSTANCE, FALSE, BOOL, TRUE, ATOM};
use winapi::um::winuser::{PostQuitMessage, DefWindowProcW, WNDCLASSEXW, CS_OWNDC, LoadCursorW, IDC_ARROW, AdjustWindowRectEx, WS_OVERLAPPEDWINDOW, CreateWindowExW, RegisterClassExW, CW_USEDEFAULT, UnregisterClassW, ShowWindow, MSG, PeekMessageW, TranslateMessage, DispatchMessageW, WM_QUIT, ShowWindowAsync, GetFocus, GetMessageW};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::_core::ptr::null_mut;
use winapi::um::winnt::LPCWSTR;
use std::io::Error;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use winapi::um::errhandlingapi::GetLastError;
use tsugumi_windows_library::{BoolInto, wide_char};

use winapi::um::winuser::{GetKeyboardLayout, GetKeyboardState, WM_DESTROY};

// extern "C" fn window_procedure(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
//     match msg {
//         WM_DESTROY => unsafe { PostQuitMessage(0) },
//         _ => return unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
//     };
//     return 0;
// }

pub struct TwHWND(pub &'static mut HWND__);
#[derive(Clone)]
pub struct ArcHWND(pub Arc<Mutex<TwHWND>>);
pub struct TwWNDCLASSEXW(WNDCLASSEXW);


pub struct TwMSG {
    value: MSG,
    pub hasMessage: bool,
}
impl TwWNDCLASSEXW{
    pub fn new(classname:&str)->Self{
        extern "system" fn window_procedure(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
            match msg {
                WM_DESTROY => unsafe {
                    println!("window destroied");
                    PostQuitMessage(0)
                },
                _ => unsafe {
                    let result = DefWindowProcW(hwnd, msg, wparam, lparam);
                    return result;
                },
            };
            return 0;
        }
        TwWNDCLASSEXW(
            WNDCLASSEXW {
                cbSize: std::mem::size_of::<WNDCLASSEXW>() as UINT,
                style: CS_OWNDC,
                lpfnWndProc: Some(window_procedure),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: unsafe { GetModuleHandleW(null_mut()) as HINSTANCE },
                hIcon: null_mut(),
                hCursor: unsafe { LoadCursorW(null_mut(), IDC_ARROW) },
                hbrBackground: null_mut(),
                lpszMenuName: null_mut(),
                lpszClassName: classname.to_wide_chars().as_ptr(),
                hIconSm: 0 as HICON,
            }
        )
    }
    pub fn tw_register_class_ex_w(&self) -> ATOM {
        unsafe{RegisterClassExW(&self.0)}
    }
    pub fn tw_unregister_class_w(&self)->bool {
        unsafe { UnregisterClassW(self.0.lpszClassName, self.0.hInstance).intobool() }
    }
}
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
                const WINDOW_WIDTH: u32 = 720;
                const WINDOW_HEIGHT: u32 = 480;
                RECT { left: 0, top: 0, right: WINDOW_WIDTH as i32, bottom: WINDOW_HEIGHT as i32 }
            }
        };
        TwHWND::tw_adjust_window_rect_ex(window_rc);
        match TwHWND::tw_create_window_ex_w_result(_wndclassexw, window_rc) {
            Ok(v) => return v,
            Err(v) => panic!("{},{}", v, unsafe { GetLastError() })
        }
    }
    fn tw_adjust_window_rect_ex(mut window_rc: RECT) {
        unsafe { AdjustWindowRectEx(&mut window_rc, WS_OVERLAPPEDWINDOW, FALSE, 0); }
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
}

impl TwMSG {
    ///ピークメッセージをを含んだMSG構造体を返す
    pub fn tw_peek_message_w(tw_hwnd: Option<&mut TwHWND>, wMsgFilterMin: UINT, wMsgFilterMax: UINT, wRemoveMsg: UINT) -> TwMSG {
        let mut msg = MSG {
            hwnd: null_mut(),
            message: 0,
            wParam: 0,
            lParam: 0,
            time: 0,
            pt: POINT { x: 0, y: 0 },
        };
        let hWnd:HWND = match tw_hwnd {
            None => {null_mut()}
            Some(v) => {v.0}
        };
        match unsafe { PeekMessageW(&mut msg, hWnd, wMsgFilterMin, wMsgFilterMax, wRemoveMsg).intobool() } {
            true => return TwMSG { value: msg, hasMessage: true },
            false => return TwMSG { value: msg, hasMessage: false }
        }
    }
    pub fn tw_get_message_w(tw_hwnd: Option<&mut TwHWND>, wMsgFilterMin: UINT, wMsgFilterMax: UINT) -> TwMSG {
        let mut msg = MSG {
            hwnd: null_mut(),
            message: 0,
            wParam: 0,
            lParam: 0,
            time: 0,
            pt: POINT { x: 0, y: 0 },
        };
        let hWnd:HWND = match tw_hwnd {
            None => {null_mut()}
            Some(v) => {v.0}
        };
        match unsafe { GetMessageW(&mut msg, hWnd, wMsgFilterMin, wMsgFilterMax).intobool() } {
            true => return TwMSG { value: msg, hasMessage: true },
            false => return TwMSG { value: msg, hasMessage: false }
        }
    }
    pub fn tw_translate_message(&self) -> bool {
        unsafe { return TranslateMessage(&self.value).intobool(); };
    }
    pub fn tw_dispatch_message_w(&self) {
        unsafe { DispatchMessageW(&self.value) };
    }
    pub fn tw_has_wm_quit_message(&self) -> bool {
        self.value.message == WM_QUIT
    }
}
