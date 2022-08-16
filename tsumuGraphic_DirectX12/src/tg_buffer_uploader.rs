use std::ffi::CString;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ptr::null_mut;
use winapi::ctypes::c_void;
use winapi::shared::minwindef::{DWORD, UINT};
use winapi::shared::winerror::S_OK;
use winapi::um::d3d12::{D3D12_COMMAND_LIST_TYPE_DIRECT, D3D12_CPU_DESCRIPTOR_HANDLE, D3D12_DESCRIPTOR_HEAP_DESC, D3D12_DESCRIPTOR_HEAP_TYPE, D3D12_FENCE_FLAG_NONE, D3D12_GPU_DESCRIPTOR_HANDLE, D3D12_GPU_VIRTUAL_ADDRESS, D3D12_RANGE, D3D12_RESOURCE_ALIASING_BARRIER, D3D12_RESOURCE_BARRIER, D3D12_RESOURCE_BARRIER_FLAGS, D3D12_RESOURCE_BARRIER_TYPE_ALIASING, D3D12_RESOURCE_BARRIER_TYPE_TRANSITION, D3D12_RESOURCE_BARRIER_TYPE_UAV, D3D12_RESOURCE_BARRIER_u, D3D12_RESOURCE_DESC, D3D12_RESOURCE_TRANSITION_BARRIER, D3D12_RESOURCE_UAV_BARRIER, ID3D12CommandAllocator, ID3D12DescriptorHeap, ID3D12Device, ID3D12Fence, ID3D12PipelineState, ID3D12Resource, ID3D12RootSignature};
use winapi::um::d3dcommon::{D3D_SHADER_MACRO, ID3D10Blob, ID3DBlob, ID3DInclude};
use winapi::um::d3dcompiler::D3DCompileFromFile;
use winapi::um::handleapi::CloseHandle;
use winapi::um::minwinbase::SECURITY_ATTRIBUTES;
use winapi::um::synchapi::{CreateEventW, WaitForSingleObject};
use winapi::um::winbase::INFINITE;
use winapi::um::winnt::{HRESULT, LPCWSTR, HANDLE, LPCSTR};
use tsugumi_windows_library::{BoolInto, wide_char,HRESULTinto};
use crate::CpID3D12CommandQueue;
use crate::tg_device::TgID3D12Device;
use crate::tg_directx::{CpEventW, CpID3D12CommandAllocator, CpID3D12Fence, CpID3D12Resource};
use crate::tg_graphics_command_list::CpID3D12GraphicsCommandList;
pub struct BufferUploadBatch {
    command_allocator:CpID3D12CommandAllocator,
    pub(crate) command_list:CpID3D12GraphicsCommandList,
    fence:CpID3D12Fence,
    event:CpEventW,
    pub(crate) resource_vec:Vec<CpID3D12Resource<u8,&'static mut [u8]>>
}

impl BufferUploadBatch {
    pub(crate) fn create_batch(device:&TgID3D12Device)->Self{
        let mut tg_command_allocators = device.cp_create_command_allocator(D3D12_COMMAND_LIST_TYPE_DIRECT).unwrap();
        //Command ListをReset()するとき、バインドするCommand Allocatorを前のCommand Allocatorと別のものに変えることができます。(https://shobomaru.wordpress.com/2015/04/20/d3d12-command/)
        let tg_command_list = device.cp_create_command_list(0, D3D12_COMMAND_LIST_TYPE_DIRECT, &mut tg_command_allocators, &mut None).unwrap();
        let mut tg_fence = device.cp_create_fence(1, D3D12_FENCE_FLAG_NONE).unwrap();
        let mut event = CpEventW::cp_create_event_w(None, false, false, None).unwrap();
        BufferUploadBatch{
            command_allocator: tg_command_allocators,
            command_list: tg_command_list,
            fence: tg_fence,
            event,
            resource_vec: vec![]
        }
    }
    pub(crate) fn end(&mut self,queue:&CpID3D12CommandQueue){
        self.command_list.cp_close();
        queue.cp_execute_command_list(&mut self.command_list);

        self.fence.cp_increment_counter(1);
        queue.cp_signal(&mut self.fence);
        if (!self.fence.cp_is_reach_fance_value()) {
            self.fence.cp_set_event_on_completion(&mut self.event);
            self.event.cp_wait_for_single_object(INFINITE);
        }
    }

}
