use std::io::Error;
use std::ptr::null_mut;
use std::thread;
use winapi::Interface;
use winapi::shared::dxgiformat::{DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_FORMAT_R8G8B8A8_UNORM_SRGB};
use winapi::shared::minwindef::{TRUE, UINT};
use winapi::shared::windef::RECT;
use winapi::shared::winerror::{HRESULT, NOERROR};
use winapi::um::d3d12::{D3D12_COMMAND_LIST_TYPE_DIRECT, D3D12_DESCRIPTOR_HEAP_DESC, D3D12_DESCRIPTOR_HEAP_FLAG_NONE, D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE, D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV, D3D12_DESCRIPTOR_HEAP_TYPE_RTV, D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER, D3D12_DESCRIPTOR_HEAP_TYPE_DSV, D3D12_FENCE_FLAG_NONE, D3D12_HEAP_FLAG_NONE, D3D12_RECT, D3D12_RENDER_TARGET_VIEW_DESC, D3D12_RENDER_TARGET_VIEW_DESC_u, D3D12_RESOURCE_STATE_PRESENT, D3D12_RESOURCE_STATE_RENDER_TARGET, D3D12_RESOURCE_TRANSITION_BARRIER, D3D12_RTV_DIMENSION_TEXTURE2D, D3D12_TEX2D_RTV, D3D12_VIEWPORT, D3D12GetDebugInterface, ID3D12PipelineState, D3D12_RESOURCE_BARRIER_FLAG_BEGIN_ONLY, D3D12_CLEAR_FLAG_DEPTH, D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT, D3D_ROOT_SIGNATURE_VERSION_1_0};
use winapi::um::d3d12sdklayers::{ID3D12Debug, ID3D12Debug1};
use winapi::um::winbase::INFINITE;
use tsugumi_windows_library::tw_hwnd::ArcHWND;
use tsugumi_windows_library::vector_Hresult;
use tsumugi::controller::TsumugiPortalPlaneLocal;
use crate::tg_camera::tsumugraphic_cameratrait;
use crate::tg_descriptor_controller::{TgD3d12DescriptorHeapDesc, TgID3D12DescriptorHeapList};
use crate::tg_device::TgID3D12Device;
use crate::tg_directx::{CpD3D12_RESOURCE_BARRIER, CpEventW, CpID3D12CommandAllocator};
use crate::tg_directx::CpD3d12ResourceBarrierDescType::CpD3d12ResourceTransitionBarrier;
use crate::tg_dxgi_factory::CpIDXGIFactory6;
use crate::tg_graphics_command_list::{CommandLists, CpID3D12GraphicsCommandList};
use crate::tg_root_signature::TgD3d12RootSignatureDesc;
use crate::TsumuGraphicObject;

const FRAME_COUNT: usize = 2;

impl TsumuGraphicObject {
    pub fn draw_window(&self,arc_hwnd: &Box<ArcHWND>, tc: &TsumugiPortalPlaneLocal) {
        let a = [1,2].iter().map(|v|{v});
        let mut thread_handle_window = arc_hwnd.clone();
        let tg_directx = self.clone();
        let mut tg_descriptor_cbv_srv_uav = self.tg_device.cp_create_descriptor_heap::<D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV>(TgD3d12DescriptorHeapDesc {
            dynamic_descriptors: 512,
            static_descriptors: 512,
            flags: D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE,
            node_mask: 0
        }).unwrap();
        let mut tg_descriptor_sampler = self.tg_device.cp_create_descriptor_heap::<D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER>(TgD3d12DescriptorHeapDesc {
            dynamic_descriptors: 512,
            static_descriptors: 512,
            flags: D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE,
            node_mask: 0
        }).unwrap();
        let mut tg_descriptor_rtv = self.tg_device.cp_create_descriptor_heap::<D3D12_DESCRIPTOR_HEAP_TYPE_RTV>(TgD3d12DescriptorHeapDesc {
            dynamic_descriptors: 2,
            static_descriptors: 0,
            flags: D3D12_DESCRIPTOR_HEAP_FLAG_NONE,
            node_mask: 0
        }).unwrap();
        let mut tg_descriptor_dsv = self.tg_device.cp_create_descriptor_heap::<D3D12_DESCRIPTOR_HEAP_TYPE_DSV>(TgD3d12DescriptorHeapDesc {
            dynamic_descriptors: 1,
            static_descriptors: 1,
            flags: D3D12_DESCRIPTOR_HEAP_FLAG_NONE,
            node_mask: 0
        }).unwrap();
        let mut tg_id3d12descriptor_heap_list = TgID3D12DescriptorHeapList{
            cbv_srv_uav: tg_descriptor_cbv_srv_uav,
            sampler: tg_descriptor_sampler,
            rtv: tg_descriptor_rtv.clone(),
            dsv: tg_descriptor_dsv
        };
        self.fetch_materialdata(&tc.tp, &mut tg_id3d12descriptor_heap_list);
        self.fetch_figuredata(&tc.tp);
        let mut camera_resource = self.fetch_cameradata(&tc.tp, &mut tg_id3d12descriptor_heap_list);
        let mut camera_thread_local = self.directx_store.camera.clone();
        thread::spawn(move || {
            let tg_device = tg_directx.tg_device.clone();
            let mut tg_command_queue = tg_directx.tg_queue.clone();
            let tg_factory = CpIDXGIFactory6::new();
            let tg_swapchain = tg_factory.cp_create_swap_chain_for_hwnd::<FRAME_COUNT>(&mut *tg_command_queue.lock().unwrap(), &mut thread_handle_window, None).unwrap_or_else(|v| { panic!("last OS error: {:?}", Error::last_os_error()) });
            let rect:RECT = thread_handle_window.clone().0.clone().lock().unwrap().tw_get_client_rect();
            drop(thread_handle_window);
            let mut currentindex = tg_swapchain.cp_get_current_back_buffer_index();
            let mut currentindex_usize = currentindex as usize;
            let mut tg_command_allocators = tg_device.cp_create_command_allocators::<3>(D3D12_COMMAND_LIST_TYPE_DIRECT).unwrap();
            //Command ListをReset()するとき、バインドするCommand Allocatorを前のCommand Allocatorと別のものに変えることができます。(https://shobomaru.wordpress.com/2015/04/20/d3d12-command/)
            let tg_command_list = tg_device.cp_create_command_lists(0, D3D12_COMMAND_LIST_TYPE_DIRECT, &mut tg_command_allocators, &mut None).unwrap();
            let mut tg_command_list = CommandLists(Vec::from(tg_command_list));
            let mut tg_handle_rtvs = tg_descriptor_rtv.allocate_dynamic_descriptor_handles::<FRAME_COUNT>().unwrap();
            let mut tg_resource_rendertarges = [tg_swapchain.cp_get_buffer(0).unwrap(), tg_swapchain.cp_get_buffer(1).unwrap()];
            for i in 0..FRAME_COUNT {
                let view_desc = D3D12_RENDER_TARGET_VIEW_DESC {
                    Format: DXGI_FORMAT_R8G8B8A8_UNORM,
                    ViewDimension: D3D12_RTV_DIMENSION_TEXTURE2D,
                    u: unsafe {
                        *std::mem::transmute::<&D3D12_TEX2D_RTV, &D3D12_RENDER_TARGET_VIEW_DESC_u>(&D3D12_TEX2D_RTV { MipSlice: 0, PlaneSlice: 0 })
                    },
                };
                tg_device.cp_create_render_target_view(&mut tg_resource_rendertarges[i], Some(view_desc), &tg_handle_rtvs[i]);
            }
            let mut depth_resorce = tg_device.tg_create_depth_stencil_resource(0, rect.right, rect.bottom).unwrap();
            let mut tg_handle_dsv = tg_id3d12descriptor_heap_list.dsv.allocate_dynamic_descriptor_handle().unwrap();
            tg_device.tg_create_depth_stencil_view(&mut depth_resorce,&tg_handle_dsv);

            let mut tg_fence = tg_device.cp_create_fence(1, D3D12_FENCE_FLAG_NONE).unwrap();
            let mut event = CpEventW::cp_create_event_w(None, false, false, None).unwrap();
            tg_command_list.tg_close();
            loop {
                //最初にロックしておく
                let command_queue = tg_command_queue.lock().unwrap();
                {
                    tg_command_allocators.iter().map(|alloc| { alloc.cp_reset() }).collect::<Vec<_>>();
                    tg_command_list.tg_reset(&mut tg_command_allocators, &mut None);
                }
                tg_command_list.tg_omset_render_targets(0..3,&vec![tg_handle_rtvs[currentindex_usize].cpu_hanle], false, Some(&tg_handle_dsv.cpu_hanle));
                tg_command_list.tg_set_descriptor_heaps(&mut tg_id3d12descriptor_heap_list);
                let mut transition_barrier_desc = D3D12_RESOURCE_TRANSITION_BARRIER {
                    pResource: tg_resource_rendertarges[currentindex_usize].interface,
                    Subresource: 0,
                    StateBefore: D3D12_RESOURCE_STATE_PRESENT,
                    StateAfter: D3D12_RESOURCE_STATE_RENDER_TARGET,
                };
                let mut barrier_desc = CpD3D12_RESOURCE_BARRIER::new(CpD3d12ResourceTransitionBarrier { d3d12_resource_transition_barrier: transition_barrier_desc, flags: 0 });
                tg_command_list.0[0].cp_resource_barrier(&vec![barrier_desc]);
                tg_command_list.0[0].cp_clear_render_target_view(&tg_handle_rtvs[currentindex_usize].cpu_hanle, &[0., 1., 1., 1.], None);
                tg_command_list.0[0].tg_clear_depth_stencil_view(&tg_handle_dsv,D3D12_CLEAR_FLAG_DEPTH , None);
                tg_command_list.0[0].cp_omset_render_targets(&vec![tg_handle_rtvs[currentindex_usize].cpu_hanle], false, None);

                camera_thread_local.lock().unwrap().update_camera_resource(&mut camera_resource);
                tg_directx.directx_store.draw_figures(&mut tg_command_list.0[1..2],&mut camera_resource);

                transition_barrier_desc.StateBefore = D3D12_RESOURCE_STATE_RENDER_TARGET;
                transition_barrier_desc.StateAfter = D3D12_RESOURCE_STATE_PRESENT;
                let barrier_desc = CpD3D12_RESOURCE_BARRIER::new(CpD3d12ResourceTransitionBarrier { d3d12_resource_transition_barrier: transition_barrier_desc, flags: 0 });
                tg_command_list.0[2].cp_resource_barrier(&vec![barrier_desc]);
                tg_command_list.tg_close();
                command_queue.cp_execute_command_lists(&mut tg_command_list.0);
                tg_swapchain.cp_present(0, 0);
                tg_fence.cp_increment_counter(1);
                command_queue.cp_signal(&mut tg_fence);
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
}