use std::ptr::null_mut;
use winapi::shared::minwindef::WPARAM;
use winapi::shared::windef::{HWND, LPPOINT, POINT};
use winapi::um::wingdi::MAKEPOINTS;
use winapi::um::winuser::{GET_WHEEL_DELTA_WPARAM, GET_XBUTTON_WPARAM, GetCursorPos, ScreenToClient};
use tsugumi_windows_library::BoolInto;
use tsugumi_windows_library::tw_hwnd::TwHWND;
use tsugumi_windows_library::tw_msg::TwMSG;

struct CursorPos{
    xPos:i32,
    yPos:i32
}
pub struct Tsumumouse{
    screenPos:CursorPos,
    windowPos:CursorPos,
}
impl Tsumumouse{

    pub fn new()->Self{
        Tsumumouse{ screenPos: CursorPos { xPos: 0, yPos: 0 }, windowPos: CursorPos { xPos: 0, yPos: 0 } }
    }
    pub fn tm_get_mouse_pos(&mut self, hWnd: &mut TwHWND){
        let mut pt:POINT = POINT{ x: 0, y: 0 };
        unsafe {
            let res = unsafe { GetCursorPos(&mut pt)}.intobool();
        }
        self.screenPos.xPos = pt.x;
        self.screenPos.yPos = pt.y;
        unsafe {
            let res = unsafe { ScreenToClient(hWnd.0, &mut pt)}.intobool();
        }

        self.windowPos.xPos = pt.x;
        self.windowPos.yPos = pt.y;
    }

}