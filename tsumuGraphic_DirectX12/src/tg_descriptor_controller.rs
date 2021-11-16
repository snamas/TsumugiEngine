use std::mem::MaybeUninit;
use std::ptr::drop_in_place;
use std::sync::{Arc, Mutex};
use winapi::shared::minwindef::UINT;
use winapi::um::d3d12::{D3D12_CPU_DESCRIPTOR_HANDLE, D3D12_DESCRIPTOR_HEAP_DESC, D3D12_DESCRIPTOR_HEAP_TYPE, D3D12_GPU_DESCRIPTOR_HANDLE, ID3D12DescriptorHeap};

pub struct TgID3D12DescriptorHeap {
    pub(crate) value: *const ID3D12DescriptorHeap,
    pub(crate) desc: D3D12_DESCRIPTOR_HEAP_DESC,
    pub(crate) align_size:UINT,
    pub(crate) tg_d3d12cpudescriptor_handle:TgD3d12CPUDescriptorHandle,
    pub(crate) tg_d3d12gpudescriptor_handle:TgD3d12GPUDescriptorHandle,
    pub(crate) descriptor_controller:Arc<Mutex<TgDescriptorController>>
}

pub struct TgD3d12CPUDescriptorHandle {
    pub(crate) value: D3D12_CPU_DESCRIPTOR_HANDLE,
    pub(crate) descriptor_heap_type: D3D12_DESCRIPTOR_HEAP_TYPE,
    pub(crate) align_size:UINT
}

pub struct TgD3d12GPUDescriptorHandle {
    pub(crate) value: D3D12_GPU_DESCRIPTOR_HANDLE,
    pub(crate) descriptor_heap_type: D3D12_DESCRIPTOR_HEAP_TYPE,
    pub(crate) align_size:UINT
}
pub struct TgDescriptorHandle{
    pub cpu_hanle:D3D12_CPU_DESCRIPTOR_HANDLE,
    pub gpu_hanle:D3D12_GPU_DESCRIPTOR_HANDLE,
    pub(crate) heap_type:D3D12_DESCRIPTOR_HEAP_TYPE,
    index:u32,
    descriptor_controller:Arc<Mutex<TgDescriptorController>>
}
pub struct TgDescriptorController{
    pub(crate) free_list:Vec<u32>,
}
pub struct TgcDescriptorHandle{
    descriptor_handle:TgDescriptorHandle,
    controller:Arc<Mutex<TgDescriptorController>>
}

impl TgD3d12CPUDescriptorHandle {
    fn tg_get_pointer(&self, index: u32) -> D3D12_CPU_DESCRIPTOR_HANDLE {
        return D3D12_CPU_DESCRIPTOR_HANDLE { ptr: self.value.ptr + (index * self.align_size) as usize };
    }
}

impl TgD3d12GPUDescriptorHandle {
    fn tg_get_pointer(&self, index: u32) -> D3D12_GPU_DESCRIPTOR_HANDLE {
        return D3D12_GPU_DESCRIPTOR_HANDLE { ptr: self.value.ptr + (index * self.align_size) as u64 };
    }
}
impl TgID3D12DescriptorHeap {
    pub fn cp_get_cpudescriptor_handle(&self) -> TgD3d12CPUDescriptorHandle {
        let descripter_heap = unsafe {
            self.value.as_ref().unwrap().GetCPUDescriptorHandleForHeapStart()
        };
        return TgD3d12CPUDescriptorHandle {
            value: descripter_heap,
            descriptor_heap_type: self.desc.Type,
            align_size: self.align_size
        };
    }
    pub fn cp_get_gpudescriptor_handle(&self)  -> TgD3d12GPUDescriptorHandle {
        let descripter_heap = unsafe {
            self.value.as_ref().unwrap().GetGPUDescriptorHandleForHeapStart()
        };
        return TgD3d12GPUDescriptorHandle {
            value: descripter_heap,
            descriptor_heap_type: self.desc.Type,
            align_size: self.align_size
        };
    }
    fn get_descriptor_handle(&self,index:u32)->TgDescriptorHandle{
        TgDescriptorHandle{
            cpu_hanle: self.tg_d3d12cpudescriptor_handle.tg_get_pointer(index),
            gpu_hanle: self.tg_d3d12gpudescriptor_handle.tg_get_pointer(index),
            heap_type: self.desc.Type,
            index,
            descriptor_controller: self.descriptor_controller.clone()
        }
    }
    pub fn allocate_descriptor_handle(&mut self)->Option<TgDescriptorHandle>{
        let index = {self.descriptor_controller.lock().unwrap().free_list.pop()?};
        Some(self.get_descriptor_handle(index))
    }
    pub fn allocate_descriptor_handles<const N: usize>(&mut self)->Option<[TgDescriptorHandle;N]>{
        let mut descriptor_handle_uninit:MaybeUninit<[TgDescriptorHandle;N]> = unsafe { MaybeUninit::uninit().assume_init() };
        for i in unsafe{ descriptor_handle_uninit.assume_init_mut()}{
            //swapで交換、forgetでdropさせないをしておかないと、未初期化領域を解放しようとしてすべてを破壊する。
            let mut handle = self.allocate_descriptor_handle()?;
            std::mem::swap(i, &mut handle);
            std::mem::forget(handle);
        }
        let descriptor_handles = unsafe{ descriptor_handle_uninit.assume_init()};
        Some(descriptor_handles)
    }
}

impl Drop for TgcDescriptorHandle{
    ///dropする際、free_listにindexを返却してからdropする
    fn drop(&mut self) {
        dbg!(&self.descriptor_handle.descriptor_controller.lock().unwrap().free_list);
        self.descriptor_handle.descriptor_controller.lock().unwrap().free_list.push(self.descriptor_handle.index);
    }
}

impl TgDescriptorController {
    ///あとどれくらいHandleを引き出せるかを示す。
    fn free_item_left(&self) -> usize {
        self.free_list.capacity()-self.free_list.len()
    }
}