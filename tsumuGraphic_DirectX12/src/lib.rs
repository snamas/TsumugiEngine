mod gpu_store;
mod tg_directx;
mod tg_device;
mod tg_command_queue;
mod tg_graphics_command_list;
mod tg_command_dispatcher;
mod Draw;
mod tg_dxgi_factory;
mod tg_dxgi_swapchain;
mod tg_descriptor_controller;
mod tg_graphics_pipeline;

use std::ops::DerefMut;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;
use winapi::_core::ptr::null_mut;
use winapi::Interface;
use winapi::shared::minwindef::TRUE;
use winapi::um::d3d12::D3D12GetDebugInterface;
use winapi::um::d3d12sdklayers::{ID3D12Debug, ID3D12Debug1};
use winapi::um::wingdi::TextOutW;
use winapi::um::winuser::{GetDC, InvalidateRect, ReleaseDC};
use tsugumi_windows_library::wide_char;
use tsumugi::controller::{TsumugiController, TsumugiController_thread, TsumugiControllerItemLifeTime, TsumugiControllerItemState, TsumugiControllerTrait, TsumugiObject};
use tsumugi::parcelreceptor_novalue::TsumugiParcelReceptorNoVal;
use tsumugi::signal::TsumugiSignal;
use tsumugiWindowController::window_hander_procedure::ArcHWND;
use tsumuObject::TsumugiObjectController;
use tsumuStockCPU::TsumugiStockController;
use crate::Draw::DrawWindow;

static CONTROLLER_NAME: &str = "tsumuGraphicDx12";

struct TsumuGraphicObject();
struct TsumuGPUStore();
fn CheckStoreList(arc_hwnd: &Box<ArcHWND>, tc: &TsumugiController_thread){
    let thread_handle = arc_hwnd.clone();
    let Object_antenna = TsumugiParcelReceptorNoVal::<TsumugiStockController>::new().subscribe(Arc::new(move |parcel|{
        let recept = parcel.parcel.clone().unwrap();
        let thread_handle = thread_handle.clone();
        thread::spawn(move||{
            loop {
                sleep(Duration::new(0, 1));
                let mut itemcount = 0;
                for item in recept.0.lock().unwrap().keys(){
                    unsafe {
                        let hdc = GetDC((*thread_handle.0.lock().unwrap()).0.deref_mut());
                        InvalidateRect((*thread_handle.0.lock().unwrap()).0.deref_mut(), null_mut(), TRUE);
                        let text = format!("path = {}", item.to_str().unwrap());
                        TextOutW(hdc, 150, 20 + itemcount, text.to_wide_chars().as_ptr(), text.len() as i32);
                        ReleaseDC((*thread_handle.0.lock().unwrap()).0.deref_mut(), hdc);
                    }
                    itemcount += 20;
                }
            }
        });
        TsumugiControllerItemState::Fulfilled
    })).to_antenna().lifetime(TsumugiControllerItemLifeTime::Once);
    tc.tc.find("TsumugiStockCPU").unwrap().recept_channel_sender.send(Object_antenna.into());
}
fn receptHWND(tc: &TsumugiController_thread) {
    let func = move |arc_hwnd: &TsumugiParcelReceptorNoVal<ArcHWND>, tct: &TsumugiController_thread| {
        let thread_handleWindow = arc_hwnd.parcel.clone().unwrap();
        {
            CheckStoreList(&thread_handleWindow, &tct);
            DrawWindow(&thread_handleWindow, &tct);
        }
        TsumugiControllerItemState::Fulfilled
    };
    let hwnd_dist = TsumugiParcelReceptorNoVal::<ArcHWND>::new().subscribe_tc(Arc::new(func)).to_antenna().lifetime(TsumugiControllerItemLifeTime::Once);
    tc.tc.find("tsumugiWindowHandle").unwrap().recept_channel_sender.send(hwnd_dist.into());
}

impl TsumugiObject for TsumuGraphicObject{
    fn on_create(&self, tc: &TsumugiController_thread) {
        receptHWND(tc);
    }
}

pub fn spown_direct_x12_handler(tc: &Box<TsumugiController>) -> Box<TsumugiController> {
    let mut newtc = tc.spown(CONTROLLER_NAME.to_string());
    newtc.set_objects(vec![
        Box::new(TsumuGraphicObject()),
    ]);
    return newtc;
}