use std::io::Error;
use std::ops::DerefMut;
use std::ptr::null_mut;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use winapi::Interface;
use winapi::um::d3d12::D3D12GetDebugInterface;
use winapi::um::d3d12sdklayers::ID3D12Debug;
use winapi::um::winuser::{FindWindowExW, FindWindowW, PM_REMOVE, SW_SHOW, VK_CONTROL};
use tsugumi_windows_library::to_wide_chars;
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
            loop {
                sleep(Duration::new(0,5));
                let mut cpmsg = TwMSG::tw_peek_message_w((*arc_handle.lock().unwrap()).0.deref_mut(), 0, 0, PM_REMOVE);
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