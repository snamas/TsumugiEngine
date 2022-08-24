
use std::io::Error;
use std::ptr::null_mut;
use std::sync::{Arc, Mutex};
use winapi::shared::minwindef::{FALSE, UINT};
use winapi::shared::windef::{HWND, HWND__, POINT, RECT};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winnt::LPCWSTR;
use winapi::um::winuser::{AdjustWindowRectEx, CreateWindowExW, CW_USEDEFAULT, DispatchMessageW, GET_WHEEL_DELTA_WPARAM, GetClientRect, GetFocus, GetMessageW, MSG, PeekMessageW, ShowWindow, ShowWindowAsync, TranslateMessage, WM_KEYDOWN, WM_MOUSEMOVE, WM_MOUSEWHEEL, WM_QUIT, WS_OVERLAPPEDWINDOW};
use crate::{BoolInto, wide_char};
use crate::tw_hwnd::TwHWND;

pub struct TwMSG {
    value: MSG,
    pub hasMessage: bool,
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
    pub fn tw_has_wm_keydown_message(&self)->bool{
        self.value.message == WM_KEYDOWN
    }
    pub fn tw_has_wm_mousewheel_message(&self)->bool{
        self.value.message == WM_MOUSEWHEEL
    }
    pub fn tw_has_wm_mousemove_message(&self) ->bool{
        self.value.message == WM_MOUSEMOVE
    }
    pub fn tw_get_wheel_delta_wparam(&self) ->i16{
        unsafe { GET_WHEEL_DELTA_WPARAM(self.value.wParam) }
    }
}
