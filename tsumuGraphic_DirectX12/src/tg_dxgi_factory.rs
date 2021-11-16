use std::ops::DerefMut;
use std::ptr::null_mut;
use winapi::Interface;
use winapi::shared::dxgi1_2::{DXGI_ALPHA_MODE_UNSPECIFIED, DXGI_SCALING_STRETCH, DXGI_SWAP_CHAIN_DESC1};
use winapi::shared::dxgi1_3::{CreateDXGIFactory2, DXGI_CREATE_FACTORY_DEBUG};
use winapi::shared::dxgi1_6::IDXGIFactory6;
use winapi::shared::dxgi::{DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH, DXGI_SWAP_EFFECT_FLIP_DISCARD};
use winapi::shared::dxgi1_5::IDXGISwapChain4;
use winapi::shared::dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM;
use winapi::shared::dxgitype::{DXGI_SAMPLE_DESC, DXGI_USAGE_BACK_BUFFER};
use winapi::shared::minwindef::UINT;
use winapi::um::d3d12::ID3D12CommandQueue;
use winapi::um::unknwnbase::IUnknown;
use winapi::um::winnt::HRESULT;
use tsugumi_windows_library::HRESULTinto;
use crate::tg_command_queue::CpID3D12CommandQueue;
use tsumugiWindowController::window_hander_procedure::ArcHWND;
use crate::tg_dxgi_swapchain::CpIDXGISwapChain4;

pub struct CpIDXGIFactory6(pub *const IDXGIFactory6);
impl CpIDXGIFactory6 {
    pub fn new() -> CpIDXGIFactory6 {
        match CpIDXGIFactory6::cp_create_dxgifactory6_result() {
            Ok(v) => return v,
            Err(v) => panic!("{}", v)
        }
    }
    fn cp_create_dxgifactory6_result() -> Result<CpIDXGIFactory6, HRESULT> {
        unsafe {
            let mut _unknownobj = null_mut();
            match CreateDXGIFactory2(DXGI_CREATE_FACTORY_DEBUG, &IDXGIFactory6::uuidof(), &mut _unknownobj).result() {
                Ok(v) => {
                    match (_unknownobj as *const IDXGIFactory6).as_ref() {
                        Some(_dxgi_factory) => return Ok(CpIDXGIFactory6(_dxgi_factory)),
                        None => return Err(v)
                    };
                }
                Err(v) => return Err(v)
            }
        }
    }
    ///NはBufferCountの値。DXGI_SWAP_CHAIN_DESC1のBufferCountにいかなる値を渡してもNが必ず優先される。
    pub fn cp_create_swap_chain_for_hwnd<const N: usize>(&self, _que: &mut CpID3D12CommandQueue, hwnd: &mut ArcHWND, dxgi_swap_chain_desc1_opt: Option<DXGI_SWAP_CHAIN_DESC1>) -> Result<CpIDXGISwapChain4<N>, HRESULT> {
        let mut dxgi_swap_chain_desc1 = match dxgi_swap_chain_desc1_opt {
            Some(v) => { v }
            None => {
                DXGI_SWAP_CHAIN_DESC1 {
                    Width: 0,
                    Height: 0,
                    Format: DXGI_FORMAT_R8G8B8A8_UNORM,
                    Stereo: i32::from(false),
                    SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
                    BufferUsage: DXGI_USAGE_BACK_BUFFER,
                    BufferCount: N as UINT,
                    Scaling: DXGI_SCALING_STRETCH,
                    SwapEffect: DXGI_SWAP_EFFECT_FLIP_DISCARD,
                    AlphaMode: DXGI_ALPHA_MODE_UNSPECIFIED,
                    Flags: DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH,
                }
            }
        };
        dxgi_swap_chain_desc1.BufferCount = N as UINT;
        unsafe {
            let mut _unknownobj = null_mut();
            match self.0.as_ref().unwrap().CreateSwapChainForHwnd((*_que).value as *mut ID3D12CommandQueue as *mut IUnknown, hwnd.0.lock().unwrap().0.deref_mut(), &dxgi_swap_chain_desc1, null_mut(), null_mut(), &mut _unknownobj).result() {
                Ok(v) => {
                    match (_unknownobj as *mut IDXGISwapChain4).as_ref() {
                        Some(_dxgi_swap_chain4) => { return Ok(CpIDXGISwapChain4 { value: _dxgi_swap_chain4, desc: dxgi_swap_chain_desc1 }); }
                        None => { return Err(v); }
                    };
                }
                Err(v) => return Err(v)
            }
        }
    }
}