use std::ptr::null_mut;
use std::thread;
use winapi::Interface;
use winapi::shared::dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM_SRGB;
use winapi::shared::minwindef::{TRUE, UINT};
use winapi::um::d3d12::{D3D12_COMMAND_LIST_TYPE_DIRECT, D3D12_DESCRIPTOR_HEAP_DESC, D3D12_DESCRIPTOR_HEAP_TYPE_RTV, D3D12_FENCE_FLAG_NONE, D3D12_HEAP_FLAG_NONE, D3D12_RENDER_TARGET_VIEW_DESC, D3D12_RENDER_TARGET_VIEW_DESC_u, D3D12_RESOURCE_STATE_PRESENT, D3D12_RESOURCE_STATE_RENDER_TARGET, D3D12_RESOURCE_TRANSITION_BARRIER, D3D12_RTV_DIMENSION_TEXTURE2D, D3D12_TEX2D_RTV, D3D12GetDebugInterface};
use winapi::um::d3d12sdklayers::{ID3D12Debug, ID3D12Debug1};
use winapi::um::winbase::INFINITE;
use tsumugi::controller::TsumugiController_thread;
use tsumugiWindowController::window_hander_procedure::ArcHWND;
use crate::tg_device::TgID3D12Device;
use crate::tg_directx::{CpD3D12_RESOURCE_BARRIER, CpEventW};
use crate::tg_directx::CpD3d12ResourceBarrierDescType::CpD3d12ResourceTransitionBarrier;
use crate::tg_dxgi_factory::CpIDXGIFactory6;
const FrameCount: usize = 2;

pub fn DrawWindow(arc_hwnd: &Box<ArcHWND>, tc: &TsumugiController_thread){
    let mut thread_handleWindow = arc_hwnd.clone();
    thread::spawn(move||{
        {
            let mut _id3d12debug = null_mut();
            let mut _id3d12debug1 = null_mut();
            unsafe {
                if D3D12GetDebugInterface(&ID3D12Debug::uuidof(), &mut _id3d12debug) == 0 {
                    if let Some(deb) = (_id3d12debug as *mut ID3D12Debug).as_ref() {
                        deb.EnableDebugLayer();
                        deb.QueryInterface(&ID3D12Debug1::uuidof(),&mut _id3d12debug1);
                        (_id3d12debug as *mut ID3D12Debug1).as_ref().unwrap().SetEnableGPUBasedValidation(TRUE);
                        deb.Release();
                        println!("OKDebug!");
                    }
                }
            }
        }
        let tg_device = TgID3D12Device::new();
        let tg_factory = CpIDXGIFactory6::new();
        let mut tg_command_queue = tg_device.cp_create_command_queue(None).unwrap();
        let tg_swapchain = tg_factory.cp_create_swap_chain_for_hwnd::<FrameCount>(&mut tg_command_queue, &mut thread_handleWindow, None).unwrap();
        drop(thread_handleWindow);
        let mut currentindex = tg_swapchain.cp_get_current_back_buffer_index();
        let mut currentindex_usize = currentindex as usize;
        let mut tg_command_allocators = tg_device.cp_create_command_allocators::<2>(D3D12_COMMAND_LIST_TYPE_DIRECT).unwrap();
        let tg_command_list = tg_device.cp_create_command_lists::<1>(0, D3D12_COMMAND_LIST_TYPE_DIRECT, tg_command_allocators.get_mut(currentindex_usize).unwrap(), &mut None).unwrap();
        let mut tg_command_list = Vec::from(tg_command_list);
        let mut tg_descriptor_rtv = tg_device.cp_create_descriptor_heap(D3D12_DESCRIPTOR_HEAP_DESC{
            Type: D3D12_DESCRIPTOR_HEAP_TYPE_RTV,
            NumDescriptors: FrameCount as UINT,
            Flags: D3D12_HEAP_FLAG_NONE,
            NodeMask: 0
        }).unwrap();
        let mut tg_handle_rtvs = tg_descriptor_rtv.allocate_descriptor_handles::<FrameCount>().unwrap();
        let mut tg_resource_rendertarges = [tg_swapchain.cp_get_buffer(0).unwrap(),tg_swapchain.cp_get_buffer(1).unwrap()];
        for i in 0..FrameCount{
            let view_desc = D3D12_RENDER_TARGET_VIEW_DESC{
                Format: DXGI_FORMAT_R8G8B8A8_UNORM_SRGB,
                ViewDimension: D3D12_RTV_DIMENSION_TEXTURE2D,
                u:unsafe {
                    *std::mem::transmute::<&D3D12_TEX2D_RTV, &D3D12_RENDER_TARGET_VIEW_DESC_u>(&D3D12_TEX2D_RTV { MipSlice: 0, PlaneSlice: 0 })
                }
            };
            tg_device.cp_create_render_target_view(&mut tg_resource_rendertarges[i], Some(view_desc), &tg_handle_rtvs[i]);
        }
        let mut tg_fence = tg_device.cp_create_fence(1,D3D12_FENCE_FLAG_NONE).unwrap();
        let mut event = CpEventW::cp_create_event_w(None, false, false, None).unwrap();
        tg_command_list[0].cp_close();
        loop {
            tg_command_allocators[currentindex_usize].cp_reset();
            tg_command_list[0].cp_reset(&mut tg_command_allocators[currentindex_usize],&mut None);
            let mut transition_barrier_desc = D3D12_RESOURCE_TRANSITION_BARRIER {
                pResource: tg_resource_rendertarges[currentindex_usize].value,
                Subresource: 0,
                StateBefore: D3D12_RESOURCE_STATE_PRESENT,
                StateAfter: D3D12_RESOURCE_STATE_RENDER_TARGET,
            };
            let barrier_desc = CpD3D12_RESOURCE_BARRIER::new(CpD3d12ResourceTransitionBarrier { d3d12_resource_transition_barrier: transition_barrier_desc, flags: 0 });
            tg_command_list[0].cp_resource_barrier(&vec![barrier_desc]);
            tg_command_list[0].cp_omset_render_targets(&vec![tg_handle_rtvs[currentindex_usize].cpu_hanle],false,None);
            tg_command_list[0].cp_clear_render_target_view(&tg_handle_rtvs[currentindex_usize].cpu_hanle,&[0.,1.,0.,1.],None);

            transition_barrier_desc.StateBefore = D3D12_RESOURCE_STATE_RENDER_TARGET;
            transition_barrier_desc.StateAfter = D3D12_RESOURCE_STATE_PRESENT;
            let barrier_desc = CpD3D12_RESOURCE_BARRIER::new(CpD3d12ResourceTransitionBarrier { d3d12_resource_transition_barrier: transition_barrier_desc, flags: 0 });
            tg_command_list[0].cp_resource_barrier(&vec![barrier_desc]);
            tg_command_list[0].cp_close();
            tg_command_queue.cp_execute_command_lists(&mut tg_command_list);
            tg_swapchain.cp_present(0,0);
            tg_fence.cp_increment_counter(1);
            tg_command_queue.cp_signal(&mut tg_fence);
            if (!tg_fence.cp_is_reach_fance_value()) {
                tg_fence.cp_set_event_on_completion(&mut event);
                event.cp_wait_for_single_object(INFINITE);
            }
            currentindex = tg_swapchain.cp_get_current_back_buffer_index();
            currentindex_usize = currentindex as usize;
        }
        event.cp_CloseHandlet();

    });
}