use std::io::Error;
use std::ops::DerefMut;
use std::ptr::null_mut;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use winapi::Interface;
use winapi::shared::minwindef::TRUE;
use winapi::um::d3d12::D3D12GetDebugInterface;
use winapi::um::d3d12sdklayers::ID3D12Debug;
use winapi::um::wingdi::TextOutW;
use winapi::um::winuser::{FindWindowExW, FindWindowW, GetDC, InvalidateRect, PM_REMOVE, ReleaseDC, SW_SHOW, VK_CONTROL};
use tsugumi_windows_library::wide_char;
use tsumugi::controller::{TsumugiController, TsumugiControllerTrait, TsumugiObject};
use tsumugi::signal::TsumugiSignal;
use tsumugiKeyboardInput::Tsumukey;
use crate::window_hander_procedure::{TwHWND, TwMSG};
pub mod window_hander_procedure;


struct TsumugiWindowObject();

impl TsumugiObject for TsumugiWindowObject {
    fn on_create(&self, tc: &TsumugiController) {
        let mut keysender = tc.global_channel_sender.pickup_channel_sender.clone();
        thread::spawn(move || {
            let mut window_handle = TwHWND::new(None, None);
            let mut arc_handle = Arc::new(Mutex::new(window_handle));
            (*arc_handle.lock().unwrap()).tw_show_window(SW_SHOW);
            let mut tsumugi_key:Tsumukey = Tsumukey::new();
            println!("{}",Error::last_os_error());
            let thread_handle = arc_handle.clone();
            thread::spawn(move ||{
                let mut iCount = 0u32;

                loop {
                    sleep(Duration::new(0,100));
                    unsafe{
                        let hdc = GetDC((*thread_handle.lock().unwrap()).0.deref_mut());

                        InvalidateRect((*thread_handle.lock().unwrap()).0.deref_mut() , null_mut() , TRUE);
                        let text = format!("Count = {}" , iCount);
                        TextOutW(hdc, 10, 10, text.to_wide_chars().as_ptr(), text.len() as i32);

                        ReleaseDC((*thread_handle.lock().unwrap()).0.deref_mut() , hdc);
                        iCount =  iCount + 1;

                    }
                }

            });
            loop {
                sleep(Duration::new(0,5));
                let mut cpmsg = TwMSG::tw_peek_message_w(None, 0, 0, PM_REMOVE);
                if cpmsg.hasMessage {
                    cpmsg.tw_translate_message();
                    cpmsg.tw_dispatch_message_w();
                }
                if cpmsg.tw_has_wm_quit_message() {
                    break;
                }
                tsumugi_key.tk_get_keyboard_state();
                //Ctrlを押した。
                if tsumugi_key.tk_key_status(VK_CONTROL){
                    println!("w");
                    keysender.send(TsumugiSignal::new("w").into());
                }
            }
        });
    }
}

pub fn spown_window_handler(tc: &Box<TsumugiController>) -> Box<TsumugiController> {
    let mut newtc = tc.spown("tsumugiWindowHandle".to_string());

    newtc.set_object(vec![
        Box::new(TsumugiWindowObject()),
    ]);
    return newtc;
}