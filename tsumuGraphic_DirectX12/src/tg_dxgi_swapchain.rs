use std::ptr::null_mut;
use winapi::Interface;
use winapi::shared::dxgi1_2::DXGI_SWAP_CHAIN_DESC1;
use winapi::shared::dxgi1_5::IDXGISwapChain4;
use winapi::shared::dxgi1_3::{CreateDXGIFactory2, DXGI_CREATE_FACTORY_DEBUG};
use winapi::shared::dxgi1_6::IDXGIFactory6;
use winapi::shared::dxgitype::DXGI_SAMPLE_DESC;
use winapi::shared::minwindef::UINT;
use winapi::um::d3d12::ID3D12Resource;
use winapi::um::winnt::HRESULT;
use tsugumi_windows_library::HRESULTinto;
use crate::tg_command_queue::CpID3D12CommandQueue;
use tsumugiWindowController::window_hander_procedure::ArcHWND;
use crate::tg_directx::CpID3D12Resource;


pub struct CpIDXGISwapChain4<const N: usize> {
    pub value: *const IDXGISwapChain4,
    pub desc: DXGI_SWAP_CHAIN_DESC1,
}

impl<const N: usize> CpIDXGISwapChain4<N> {
    pub fn cp_get_desc1(&self) -> Result<DXGI_SWAP_CHAIN_DESC1, HRESULT> {
        unsafe {
            let mut dxgi_swap_chain_desc1: DXGI_SWAP_CHAIN_DESC1 = DXGI_SWAP_CHAIN_DESC1 {
                Width: 0,
                Height: 0,
                Format: 0,
                Stereo: 0,
                SampleDesc: DXGI_SAMPLE_DESC { Count: 0, Quality: 0 },
                BufferUsage: 0,
                BufferCount: 0,
                Scaling: 0,
                SwapEffect: 0,
                AlphaMode: 0,
                Flags: 0,
            };
            match self.value.as_ref().unwrap().GetDesc1(&mut dxgi_swap_chain_desc1).result() {
                Ok(_) => return Ok(dxgi_swap_chain_desc1),
                Err(v) => Err(v)
            }
        }
    }
    ///バッファの取得。indexに与えた番号のリソースが返ってくる（CpID3D12Resourceのsizeは0）
    pub fn cp_get_buffer(&self, index: UINT) -> Result<CpID3D12Resource<UINT,UINT>, HRESULT> {
        unsafe {
            let mut _unknownobj = null_mut();
            match self.value.as_ref().unwrap().GetBuffer(index, &ID3D12Resource::uuidof(), &mut _unknownobj).result() {
                Ok(v) => {
                    match (_unknownobj as *mut ID3D12Resource).as_mut() {
                        Some(id3d12resource) => { return Ok(CpID3D12Resource { interface: id3d12resource, bytesize:0, root_parameter_index: None, mapvalue: None, _phantom_s: Default::default() }); }
                        None => { return Err(v); }
                    }
                }
                Err(v) => return Err(v)
            }
        }
    }
    pub fn tg_get_buffers(&self)-> Result<CpID3D12Resource<UINT,UINT>, HRESULT> {
        todo!();
    }
    pub fn cp_get_current_back_buffer_index(&self) -> u32 {
        unsafe {
            return self.value.as_ref().unwrap().GetCurrentBackBufferIndex();
        }
    }
    pub fn cp_present(&self, SyncInterval: u32, Flags: u32) -> Result<HRESULT, HRESULT> {
        unsafe {
            self.value.as_ref().unwrap().Present(SyncInterval, Flags).result()
        }
    }
}
