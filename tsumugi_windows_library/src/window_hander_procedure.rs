use winapi::shared::windef::{HWND, RECT, HWND__, HICON, POINT};
use winapi::shared::minwindef::{UINT, WPARAM, LPARAM, LRESULT, HINSTANCE, FALSE, BOOL, TRUE, ATOM};
use winapi::um::winuser::{PostQuitMessage, DefWindowProcW, WNDCLASSEXW, CS_OWNDC, LoadCursorW, IDC_ARROW, AdjustWindowRectEx, WS_OVERLAPPEDWINDOW, CreateWindowExW, RegisterClassExW, CW_USEDEFAULT, UnregisterClassW, ShowWindow, MSG, PeekMessageW, TranslateMessage, DispatchMessageW, WM_QUIT, ShowWindowAsync, GetFocus, GetMessageW, GetClientRect, WM_KEYDOWN, WM_MOUSEWHEEL, WM_MOUSEMOVE};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::_core::ptr::null_mut;
use winapi::um::winnt::LPCWSTR;
use std::io::Error;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use winapi::um::errhandlingapi::GetLastError;

use winapi::um::winuser::{GetKeyboardLayout, GetKeyboardState, WM_DESTROY};
use crate::{BoolInto, wide_char};


pub struct TwWNDCLASSEXW(pub(crate) WNDCLASSEXW);

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
