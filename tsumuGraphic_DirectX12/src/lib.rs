extern crate core;

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
mod tg_buffer_uploader;
mod tg_camera;

use std::ops::DerefMut;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::collections::HashMap;
use std::io::Error;
use std::thread::sleep;
use std::time::Duration;
use nalgebra::Translation3;
use winapi::_core::ptr::null_mut;
use winapi::Interface;
use winapi::shared::minwindef::TRUE;
use winapi::um::d3d12::D3D12GetDebugInterface;
use winapi::um::d3d12sdklayers::{ID3D12Debug, ID3D12Debug1};
use winapi::um::wingdi::TextOutW;
use winapi::um::winuser::{GetDC, InvalidateRect, ReleaseDC};
use tsugumi_windows_library::tw_hwnd::ArcHWND;
use tsugumi_windows_library::wide_char;
use tsumugi::controller::{TsumugiPortal, TsumugiPortalPlaneLocal, TsumugiControllerItemLifeTime, TsumugiControllerItemState, TsumugiControllerTrait, TsumugiObject};
use tsumugi::parcelreceptor_novalue::TsumugiParcelReceptorNoVal;
use tsumugi::signal::TsumugiSignal;
use tsumuObject::camera::Camera;
use crate::tg_command_queue::CpID3D12CommandQueue;
use crate::tg_device::TgID3D12Device;
use crate::tsumuGPUStoreList::TsumuGPUStoreList;

static CONTROLLER_NAME: &str = "tsumuGraphicDx12";
#[derive(Clone)]
pub struct TsumuGraphicObject {
    directx_store: TsumuGPUStoreList,
    pub(crate) tg_device :Arc<TgID3D12Device>,
    pub(crate) tg_queue :Arc<Mutex<CpID3D12CommandQueue>>,
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

pub fn spawn_direct_x12_handler(tc: &Box<TsumugiPortal>) -> Box<TsumugiPortal> {
    let mut newtc = tc.spawn(CONTROLLER_NAME.to_string());
    let mut Device = Arc::new(TgID3D12Device::new());
    let mut tg_command_queue = Arc::new(Mutex::new((*Device).cp_create_command_queue(None).unwrap_or_else(|v| { panic!("last OS error: {:?}", Error::last_os_error()) })));
    let mut camera:Camera= Default::default();
    camera.position = Translation3::new(0f32,0f32,-3f32);
    newtc.set_objects(vec![
        Box::new(TsumuGraphicObject { directx_store: TsumuGPUStoreList { list: Arc::new(Mutex::new(Default::default())), camera: Arc::new(Mutex::new(camera)) }, tg_device: Device, tg_queue: tg_command_queue }),
    ]);
    return newtc;
}