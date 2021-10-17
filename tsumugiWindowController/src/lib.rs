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
use winapi::um::d3d12::D3D12GetDebugInterface;
use winapi::um::d3d12sdklayers::ID3D12Debug;
use winapi::um::wingdi::TextOutW;
use winapi::um::winuser::{FindWindowExW, FindWindowW, GetDC, InvalidateRect, PM_REMOVE, ReleaseDC, SW_SHOW, VK_CONTROL};
use tsugumi_windows_library::wide_char;
use tsumugi::antenna_chain::{TsumugiAntennaType, TsumugiReceptorChain};
use tsumugi::controller::{TsumugiController, TsumugiController_thread, TsumugiControllerItemLifeTime, TsumugiControllerItemState, TsumugiControllerTrait, TsumugiObject};
use tsumugi::distributor::TsumugiDistributor;
use tsumugi::distributor::TsumugiParcelDistributor;
use tsumugi::parcel_receptor::TsumugiParcelReceptor;
use tsumugi::parcelreceptor_novalue::TsumugiParcelReceptorNoVal;
use tsumugi::signal::TsumugiSignal;
use tsumugiKeyboardInput::Tsumukey;
use crate::window_hander_procedure::{ArcHWND, TwHWND, TwMSG};

pub mod window_hander_procedure;

static Controller_name: &str = "tsumugiWindowHandle";


struct TsumugiWindowObject();

static COUNT: AtomicU64 = AtomicU64::new(0);

fn pushW(thread_handle: Box<ArcHWND>, globalreceptor: Arc<Sender<TsumugiAntennaType>>) {
    {
        let hwnd_dist = TsumugiSignal::new("w").subscribe(Arc::new(move || {
            unsafe {
                let hdc = GetDC((*thread_handle.0.lock().unwrap()).0.deref_mut());
                InvalidateRect((*thread_handle.0.lock().unwrap()).0.deref_mut(), null_mut(), TRUE);
                let text = format!("Count = {}", COUNT.load(Ordering::SeqCst));
                TextOutW(hdc, 10, 10, text.to_wide_chars().as_ptr(), text.len() as i32);

                ReleaseDC((*thread_handle.0.lock().unwrap()).0.deref_mut(), hdc);
                COUNT.fetch_add(1, Ordering::SeqCst);
            }
            TsumugiControllerItemState::Fulfilled
        })).lifetime(TsumugiControllerItemLifeTime::Eternal);
        globalreceptor.send(hwnd_dist.into());
    }
}

fn receptHWND(tc: &TsumugiController_thread) {
    let func = move |arc_hwnd: &TsumugiParcelReceptorNoVal<ArcHWND>, tct: &TsumugiController_thread| {
        let thread_handle = arc_hwnd.parcel.clone().unwrap();
        {
            let hwnd_dist = TsumugiSignal::new("w").subscribe(Arc::new(move || {
                unsafe {
                    let hdc = GetDC((*thread_handle.0.lock().unwrap()).0.deref_mut());
                    InvalidateRect((*thread_handle.0.lock().unwrap()).0.deref_mut(), null_mut(), TRUE);
                    let text = format!("Count = {}", COUNT.load(Ordering::SeqCst));
                    TextOutW(hdc, 10, 10, text.to_wide_chars().as_ptr(), text.len() as i32);

                    ReleaseDC((*thread_handle.0.lock().unwrap()).0.deref_mut(), hdc);
                    COUNT.fetch_add(1, Ordering::SeqCst);
                }
                TsumugiControllerItemState::Fulfilled
            })).lifetime(TsumugiControllerItemLifeTime::Eternal);
            tct.tc.global_channel_sender.recept_channel_sender.send(hwnd_dist.into());
        }
        TsumugiControllerItemState::Fulfilled
    };
    let hwnd_dist = TsumugiParcelReceptorNoVal::<ArcHWND>::new().subscribe_tc(Arc::new(func)).to_antenna().lifetime(TsumugiControllerItemLifeTime::Once);
    tc.tc.local_channel_sender.recept_channel_sender.send(hwnd_dist.into());
}

impl TsumugiObject for TsumugiWindowObject {
    fn on_create(&self, tc: &TsumugiController_thread) {
        let mut globalsender = tc.tc.global_channel_sender.pickup_channel_sender.clone();
        let mut globalreceptor = tc.tc.global_channel_sender.recept_channel_sender.clone();
        let mut localsender = tc.tc.local_channel_sender.pickup_channel_sender.clone();
        thread::spawn(move || {
            let mut window_handle = TwHWND::new(None, None);
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
                    println!("w");
                    globalsender.send(TsumugiSignal::new("w").into());
                }
            }
        });
        receptHWND(tc);
    }
}

pub fn spown_window_handler(tc: &Box<TsumugiController>) -> Box<TsumugiController> {
    let mut newtc = tc.spown("tsumugiWindowHandle".to_string());
    newtc.set_objects(vec![
        Box::new(TsumugiWindowObject()),
    ]);
    return newtc;
}