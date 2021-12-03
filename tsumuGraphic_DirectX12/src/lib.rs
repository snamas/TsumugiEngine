pub mod gpu_figure_store;
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
mod tsumuGPUStoreList;
mod shader_store;

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
use crate::tg_device::TgID3D12Device;
use crate::tsumuGPUStoreList::TsumuGPUStoreList;

static CONTROLLER_NAME: &str = "tsumuGraphicDx12";
#[derive(Clone)]
pub struct TsumuGraphicObject(TsumuGPUStoreList);
impl TsumuGraphicObject {
    fn receptHWND(&self,tc: &TsumugiController_thread) {
        let thread_object_list = self.clone();
        let func = move |arc_hwnd: &TsumugiParcelReceptorNoVal<ArcHWND>, tct: &TsumugiController_thread| {
            let thread_handleWindow = arc_hwnd.parcel.clone().unwrap();
            {
                thread_object_list.draw_window(&thread_handleWindow, &tct);
            }
            TsumugiControllerItemState::Fulfilled
        };
        let hwnd_dist = TsumugiParcelReceptorNoVal::<ArcHWND>::new().subscribe_tc(Arc::new(func)).to_antenna().displayname("DirectX12ReceptHWND").lifetime(TsumugiControllerItemLifeTime::Once);
        tc.tc.find("tsumugiWindowHandle").unwrap().recept_channel_sender.send(hwnd_dist.into());
    }
}
impl TsumugiObject for TsumuGraphicObject{
    fn on_create(&self, tc: &TsumugiController_thread) {
        self.0.fetch_figuredata(&tc.tc);
        self.receptHWND(tc);
        self.0.debug_GPUStore(&tc.tc);
    }
}

pub fn spown_direct_x12_handler(tc: &Box<TsumugiController>) -> Box<TsumugiController> {
    let mut newtc = tc.spown(CONTROLLER_NAME.to_string());
    newtc.set_objects(vec![
        Box::new(TsumuGraphicObject(TsumuGPUStoreList{ list: Arc::new(Mutex::new(Default::default())), tg_device: Arc::new(TgID3D12Device::new()) })),
    ]);
    return newtc;
}