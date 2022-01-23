use winapi::um::d3d12::{D3D12_COMMAND_LIST_TYPE, ID3D12CommandList, ID3D12CommandQueue};
use winapi::um::winnt::HRESULT;
use tsugumi_windows_library::HRESULTinto;
use crate::tg_directx::CpID3D12Fence;
use crate::tg_graphics_command_list::CpID3D12GraphicsCommandList;

pub struct CpID3D12CommandQueue{
    pub(crate) value: *mut ID3D12CommandQueue,
    pub(crate) type_: D3D12_COMMAND_LIST_TYPE,
}

impl CpID3D12CommandQueue {
    pub fn cp_execute_command_lists(&self, cp_id3d12command_lists: &mut Vec<CpID3D12GraphicsCommandList>) {
        let NumCommandLists: u32 = cp_id3d12command_lists.len() as u32;
        let ppCommandLists = cp_id3d12command_lists.as_ptr() as *const *mut ID3D12CommandList;
        unsafe {
            self.value.as_ref().unwrap().ExecuteCommandLists(NumCommandLists, ppCommandLists);
        }
    }
    pub fn cp_signal(&self, cp_id3d12fence: &mut CpID3D12Fence) -> Result<HRESULT, HRESULT> {
        unsafe {
            self.value.as_ref().unwrap().Signal(cp_id3d12fence.interface, cp_id3d12fence.fenceval).result()
        }
    }
}
