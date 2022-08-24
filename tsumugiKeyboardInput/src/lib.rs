mod mouseInput;

use std::io::Error;
use winapi::um::winuser::{GetKeyboardLayout, GetKeyboardState, VK_RETURN, VK_SHIFT, VK_CONTROL, SetFocus, GetForegroundWindow, GetWindowThreadProcessId};
use winapi::shared::minwindef::{HKL, PBYTE, BYTE, BOOL};
use winapi::ctypes::{c_uchar, c_int};
use std::ops::BitAnd;
use tsumugi::controller::{TsumugiObject, TsumugiPortal, TsumugiControllerItemState, TsumugiControllerItemLifeTime, TsumugiControllerTrait};
use std::thread;
use tsumugi::signal::TsumugiSignal;
use std::sync::{Condvar, Mutex};
use std::thread::sleep;
use std::time::Duration;
use tsugumi_windows_library::BoolInto;

pub struct Tsumukey([u8;256]);
impl Tsumukey{
    pub fn new()->Self{
        Tsumukey([0;256])
    }
    pub fn tk_get_keyboard_state(&mut self) -> bool {
        unsafe {
            GetKeyboardState(self.0.as_mut_ptr()).intobool()
        }
    }
    pub fn tk_key_status(&self, vkey:c_int) ->bool{
        match self.0.get(vkey.unsigned_abs() as usize) {
            None => {panic!("")}
            Some(keystatus) => {
                //上位Ⅰビットが1ならキーが押されている。
                matches!(keystatus.bitand(128u8), 128u8)
            }
        }
    }
}

struct TsumugiInputKeyObject();

impl TsumugiObject for TsumugiInputKeyObject {
    fn on_create(&self, tct: &tsumugi::controller::TsumugiPortalPlaneLocal) {
        let mut tsumugi_key:Tsumukey = Tsumukey([0;256]);
        let mut keysender = tct.tp.global_channel_sender.pickup_channel_sender.clone();
        thread::spawn(move ||{
            loop {
                let mut  nummPID = 0u32;
                unsafe{
                    let handle = GetForegroundWindow();
                    let targetPID = GetWindowThreadProcessId(handle,&mut nummPID);

                    SetFocus(handle);
                    println!("{}",Error::last_os_error());
                };
                sleep(Duration::new(1,0));
                println!("once");
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

pub fn spown_windows_key_controller(tc: &Box<TsumugiPortal>) -> Box<TsumugiPortal> {
    let mut newtc = tc.spown("tsumugiKeyController".to_string());

    newtc.set_objects(vec![
        Box::new(TsumugiInputKeyObject()),
    ]);
    return newtc;
}