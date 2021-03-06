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
mod material_loader;
mod tg_root_signature;
mod tg_sampler;

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
use tsumugi::controller::{TsumugiPortal, TsumugiPortalPlaneLocal, TsumugiControllerItemLifeTime, TsumugiControllerItemState, TsumugiControllerTrait, TsumugiObject};
use tsumugi::parcelreceptor_novalue::TsumugiParcelReceptorNoVal;
use tsumugi::signal::TsumugiSignal;
use tsumugiWindowController::window_hander_procedure::ArcHWND;
use crate::tg_device::TgID3D12Device;
use crate::tsumuGPUStoreList::TsumuGPUStoreList;

static CONTROLLER_NAME: &str = "tsumuGraphicDx12";
#[derive(Clone)]
pub struct TsumuGraphicObject {
    directx_store: TsumuGPUStoreList,
    pub(crate) tg_device :Arc<TgID3D12Device>
}

impl TsumuGraphicObject {
    fn receptHWND(&self,tc: &TsumugiPortalPlaneLocal) {
        let thread_object_list = self.clone();
        let func = move |arc_hwnd: &TsumugiParcelReceptorNoVal<ArcHWND>, tct: &TsumugiPortalPlaneLocal| {
            let thread_handleWindow = arc_hwnd.parcel.clone().unwrap();
            {
                thread_object_list.draw_window(&thread_handleWindow, &tct);
            }
            TsumugiControllerItemState::Fulfilled
        };
        let hwnd_dist = TsumugiParcelReceptorNoVal::<ArcHWND>::new().subscribe_with_portal(Arc::new(func)).to_antenna().displayname("DirectX12ReceptHWND").lifetime(TsumugiControllerItemLifeTime::Once);
        tc.tp.find("tsumugiWindowHandle").unwrap().recept_channel_sender.send(hwnd_dist.into());
    }
}
impl TsumugiObject for TsumuGraphicObject{
    fn on_create(&self, tc: &TsumugiPortalPlaneLocal) {
        self.receptHWND(tc);
        self.directx_store.debug_GPUStore(&tc.tp);
    }
}

pub fn spown_direct_x12_handler(tc: &Box<TsumugiPortal>) -> Box<TsumugiPortal> {
    let mut newtc = tc.spown(CONTROLLER_NAME.to_string());
    let mut Device = Arc::new(TgID3D12Device::new());
    newtc.set_objects(vec![
        Box::new(TsumuGraphicObject { directx_store: TsumuGPUStoreList { list: Arc::new(Mutex::new(Default::default())) }, tg_device: Device }),
    ]);
    return newtc;
}