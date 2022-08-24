use std::io::Error;
use std::ops::DerefMut;
use std::ptr::null_mut;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use winapi::Interface;
use winapi::shared::minwindef::TRUE;
use winapi::shared::windef::RECT;
use winapi::um::d3d12::D3D12GetDebugInterface;
use winapi::um::d3d12sdklayers::ID3D12Debug;
use winapi::um::wingdi::TextOutW;
use winapi::um::winnt::LONG;
use winapi::um::winuser::{FindWindowExW, FindWindowW, GetDC, InvalidateRect, PM_REMOVE, ReleaseDC, SW_SHOW, VK_CONTROL};
use tsugumi_windows_library::tw_hwnd::TwHWND;
use tsugumi_windows_library::tw_msg::TwMSG;
use tsugumi_windows_library::wide_char;
use tsumugi::antenna_chain::{TsumugiAntennaType, TsumugiReceptorChain};
use tsumugi::controller::{TsumugiPortal, TsumugiPortalPlaneLocal, TsumugiControllerItemLifeTime, TsumugiControllerItemState, TsumugiControllerTrait, TsumugiObject};
use tsumugi::distributor::TsumugiDistributor;
use tsumugi::distributor::TsumugiParcelDistributor;
use tsumugi::parcel_receptor::TsumugiParcelReceptor;
use tsumugi::parcelreceptor_novalue::TsumugiParcelReceptorNoVal;
use tsumugi::signal::TsumugiSignal;
use tsumugiKeyboardInput::Tsumukey;
use tsugumi_windows_library::tw_hwnd::ArcHWND;
use tsumugi::controller::TsumugiControllerItemLifeTime::Flash;

static Controller_name: &str = "tsumugiWindowHandle";


struct TsumugiWindowObject();

static COUNT: AtomicU64 = AtomicU64::new(0);
const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;
impl TsumugiObject for TsumugiWindowObject {
    fn on_create(&self, tc: &TsumugiPortalPlaneLocal) {
        let mut globalsender = tc.tp.global_channel_sender.pickup_channel_sender.clone();
        let mut globalreceptor = tc.tp.global_channel_sender.recept_channel_sender.clone();
        let mut localsender = tc.tp.local_channel_sender.pickup_channel_sender.clone();
        thread::spawn(move || {
            let mut window_handle = TwHWND::new(None,
                                                Some(
                                                    RECT { left: 0, top: 0, right: WINDOW_WIDTH as LONG, bottom: WINDOW_HEIGHT as LONG }));
            let mut arc_handle = Arc::new(Mutex::new(window_handle));
            (*arc_handle.lock().unwrap()).tw_show_window(SW_SHOW);
            let handle_dist = TsumugiParcelDistributor::new(ArcHWND { 0: arc_handle.clone() }).lifetime(TsumugiControllerItemLifeTime::Eternal);
            localsender.send(handle_dist.into());
            let mut tsumugi_key: Tsumukey = Tsumukey::new();
            println!("{}", Error::last_os_error());
            loop {
                let mut cpmsg = TwMSG::tw_get_message_w(None, 0, 0);
                if cpmsg.hasMessage {
                    cpmsg.tw_translate_message();
                    cpmsg.tw_dispatch_message_w();
                }
                if cpmsg.tw_has_wm_quit_message() {
                    break;
                }
                tsumugi_key.tk_get_keyboard_state();
                //2を押した。
                if tsumugi_key.tk_key_status(87) {
                    let sender:TsumugiDistributor = TsumugiSignal::new("w").lifetime(Flash).into();
                    globalsender.send(sender);
                }
                if(cpmsg.tw_has_wm_mousewheel_message()){
                    let delta = cpmsg.tw_get_wheel_delta_wparam();
                    let sender:TsumugiDistributor = TsumugiDistributor::TPDistributor(TsumugiParcelDistributor::new(delta).parcelname("mouse_wheel").displayname("mouse_wheel").lifetime(Flash));
                    globalsender.send(sender);
                }
            }
        });
    }
}

pub fn spown_window_handler(tc: &Box<TsumugiPortal>) -> Box<TsumugiPortal> {
    let mut newtc = tc.spown("tsumugiWindowHandle".to_string());
    newtc.set_objects(vec![
        Box::new(TsumugiWindowObject()),
    ]);
    return newtc;
}