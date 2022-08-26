use std::collections::VecDeque;
use std::mem::MaybeUninit;
use std::process::id;
use std::ptr::drop_in_place;
use std::sync::{Arc, Mutex};
use winapi::shared::minwindef::UINT;
use winapi::um::d3d12::{D3D12_CPU_DESCRIPTOR_HANDLE, D3D12_DESCRIPTOR_HEAP_DESC, D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE, D3D12_DESCRIPTOR_HEAP_FLAGS, D3D12_DESCRIPTOR_HEAP_TYPE, D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV, D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER,D3D12_DESCRIPTOR_HEAP_TYPE_RTV,D3D12_DESCRIPTOR_HEAP_TYPE_DSV,D3D12_GPU_DESCRIPTOR_HANDLE, ID3D12DescriptorHeap};

pub(crate) struct TgD3d12DescriptorHeapDesc {
    pub(crate) dynamic_descriptors: u32,
    pub(crate) static_descriptors: u32,
    pub(crate) flags: D3D12_DESCRIPTOR_HEAP_FLAGS,
    pub(crate) node_mask: UINT,
}
#[derive(Clone)]
pub struct TgID3D12DescriptorHeapList{
    pub(crate) cbv_srv_uav:TgID3D12DescriptorHeap<D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV>,
    pub(crate) sampler:TgID3D12DescriptorHeap<D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER>,
    pub(crate) rtv:TgID3D12DescriptorHeap<D3D12_DESCRIPTOR_HEAP_TYPE_RTV>,
    pub(crate) dsv:TgID3D12DescriptorHeap<D3D12_DESCRIPTOR_HEAP_TYPE_DSV>,
    //D3D12_DESCRIPTOR_HEAP_TYPE_NUM_TYPESはディスクリプタヒープの種類の数を表しているだけの数字（4種類あるので4）
}
#[derive(Clone)]
pub struct TgID3D12DescriptorHeap<const heap_type: D3D12_DESCRIPTOR_HEAP_TYPE> {
    pub(crate) value: *mut ID3D12DescriptorHeap,
    pub(crate) dynamic_descriptor_number: u32,
    pub(crate) static_descriptor_number: u32,
    pub(crate) align_size: UINT,
    pub(crate) tg_d3d12cpudescriptor_handle: TgD3d12CPUDescriptorHandle,
    pub(crate) tg_d3d12gpudescriptor_handle: TgD3d12GPUDescriptorHandle,
    pub(crate) descriptor_controller: Arc<Mutex<TgDescriptorController>>,
}
//大丈夫な気がする
unsafe impl<const heap_type: D3D12_DESCRIPTOR_HEAP_TYPE> Send for TgID3D12DescriptorHeap<heap_type> {}
unsafe impl<const heap_type: D3D12_DESCRIPTOR_HEAP_TYPE> Sync for TgID3D12DescriptorHeap<heap_type> {}
#[derive(Clone, Copy)]
pub struct TgD3d12CPUDescriptorHandle {
    pub(crate) value: D3D12_CPU_DESCRIPTOR_HANDLE,
    pub(crate) descriptor_heap_type: D3D12_DESCRIPTOR_HEAP_TYPE,
    pub(crate) align_size: UINT,
}

#[derive(Clone, Copy)]
pub struct TgD3d12GPUDescriptorHandle {
    pub(crate) value: D3D12_GPU_DESCRIPTOR_HANDLE,
    pub(crate) descriptor_heap_type: D3D12_DESCRIPTOR_HEAP_TYPE,
    pub(crate) align_size: UINT,
}

#[derive(Clone)]
pub struct TgDescriptorHandle<const heap_type: D3D12_DESCRIPTOR_HEAP_TYPE> {
    pub cpu_hanle: D3D12_CPU_DESCRIPTOR_HANDLE,
    pub gpu_hanle: D3D12_GPU_DESCRIPTOR_HANDLE,
    pub(crate) heap_type: D3D12_DESCRIPTOR_HEAP_TYPE,
    pub(crate) dynamic_descriptor_number: u32,
    index: u32,
    descriptor_controller: Arc<Mutex<TgDescriptorController>>,
}
#[derive(Copy, Clone)]
struct Node{
    no:u32,
    count:u32,
    is_empty:bool
}
pub struct TgDescriptorController {
    list:VecDeque<Node>,
    static_position: u32
}

impl TgDescriptorController {
    pub(crate) fn new(dynamic_descriptors:u32) ->Self{
        let mut deq = VecDeque::new();
        deq.push_back(Node{
            no: 0,
            count: dynamic_descriptors,
            is_empty: true
        });
        TgDescriptorController{ list:deq, static_position: 0 }
    }
    fn allocate(&mut self,quantity_to_allocate:u32) -> Option<u32>{
        let find_node_index = self.list.iter_mut().position(|node|node.is_empty&&node.count >= quantity_to_allocate)?;
        let node = self.list.get_mut(find_node_index)?;
        node.is_empty = false;
        let remain_quanta = node.count-quantity_to_allocate;
        let node_number = node.no;
        if remain_quanta>0 {
            let remain_node = Node{
                no: node.no + quantity_to_allocate,
                count: remain_quanta,
                is_empty: true
            };
            node.count = quantity_to_allocate;
            self.list.insert(find_node_index+1,remain_node);
        }
        return Some(node_number);
    }
    fn free(&mut self,descriptor_number:u32)->Option<()>{
        let find_node_index = self.list.iter_mut().position(|node|!node.is_empty&&node.no == descriptor_number)?;
        let next_node = self.list.get(find_node_index+1).copied();
        let prev_node = self.list.get(find_node_index-1).copied();
        let node = self.list.get_mut(find_node_index)?;
        node.is_empty = true;
        match next_node{
            None => {}
            Some(next) => {
                if(next.is_empty){
                    node.count += next.count;
                    self.list.remove(find_node_index+1);
                }
            }
        }
        let node = self.list.get_mut(find_node_index)?;
        match prev_node{
            None => {}
            Some(prev) => {
                if(prev.is_empty){
                    node.no = prev.no;
                    node.count += prev.count;
                    self.list.remove(find_node_index-1);
                }
            }
        }
        return Some(());
    }
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
impl<const heap_type: D3D12_DESCRIPTOR_HEAP_TYPE> TgID3D12DescriptorHeap<heap_type> {
    pub fn cp_get_cpudescriptor_handle(&self) -> TgD3d12CPUDescriptorHandle {
        let descripter_heap = unsafe {
            self.value.as_ref().unwrap().GetCPUDescriptorHandleForHeapStart()
        };
        return TgD3d12CPUDescriptorHandle {
            value: descripter_heap,
            descriptor_heap_type: heap_type,
            align_size: self.align_size,
        };
    }
    pub fn cp_get_gpudescriptor_handle(&self) -> TgD3d12GPUDescriptorHandle {
        let descripter_heap = unsafe {
            self.value.as_ref().unwrap().GetGPUDescriptorHandleForHeapStart()
        };
        return TgD3d12GPUDescriptorHandle {
            value: descripter_heap,
            descriptor_heap_type: heap_type,
            align_size: self.align_size,
        };
    }
    ///indexに入れた場所のメモリを取得するよ。
    fn get_descriptor_handle(&self, index: u32) -> TgDescriptorHandle<heap_type> {
        TgDescriptorHandle {
            cpu_hanle: self.tg_d3d12cpudescriptor_handle.tg_get_pointer(index),
            gpu_hanle: self.tg_d3d12gpudescriptor_handle.tg_get_pointer(index),
            heap_type: heap_type,
            dynamic_descriptor_number: self.dynamic_descriptor_number,
            index,
            descriptor_controller: self.descriptor_controller.clone(),
        }
    }
    ///TgDescriptorHandleを返す関数だよ。ただしメモリの場所は適当。Noneの場合、すべてのTgDescriptorHandleを使い切ったことになる。
    pub fn allocate_dynamic_descriptor_handle(&mut self) -> Option<TgDescriptorHandle<heap_type>> {
        let index = { self.descriptor_controller.lock().unwrap().allocate(1)? };
        Some(self.get_descriptor_handle(index))
    }
    ///これで持ってきたハンドルは連続したメモリにあるよ。ただしメモリの使い回しが出来ないのでなくなったらおしまい。
    pub fn allocate_static_descriptor_handle<const N: usize>(&mut self) -> Option<[TgDescriptorHandle<heap_type>; N]> {
        let mut descriptor_controller = self.descriptor_controller.lock().unwrap();
        let index =  descriptor_controller.static_position;
        let mut idx = 0;
        let mut descriptor_handle_uninit: MaybeUninit<[TgDescriptorHandle<heap_type>; N]> = unsafe { MaybeUninit::uninit().assume_init() };
        for i in unsafe { descriptor_handle_uninit.assume_init_mut() } {
            //swapで交換、forgetでdropさせないをしておかないと、未初期化領域を解放しようとしてすべてを破壊する。
            let mut handle = self.get_descriptor_handle(index+idx);
            std::mem::swap(i, &mut handle);
            std::mem::forget(handle);
            idx += 1;
        }
        let descriptor_handles = unsafe { descriptor_handle_uninit.assume_init() };
        descriptor_controller.static_position += N as u32;
        Some(descriptor_handles)
    }
    ///これで持ってきたハンドルは連続したメモリにない可能性があるよ
    pub fn allocate_dynamic_descriptor_handles<const N: usize>(&mut self) -> Option<[TgDescriptorHandle<heap_type>; N]> {
        let mut descriptor_handle_uninit: MaybeUninit<[TgDescriptorHandle<heap_type>; N]> = unsafe { MaybeUninit::uninit().assume_init() };
        for i in unsafe { descriptor_handle_uninit.assume_init_mut() } {
            //swapで交換、forgetでdropさせないをしておかないと、未初期化領域を解放しようとしてすべてを破壊する。
            let mut handle = self.allocate_dynamic_descriptor_handle()?;
            std::mem::swap(i, &mut handle);
            std::mem::forget(handle);
        }
        let descriptor_handles = unsafe { descriptor_handle_uninit.assume_init() };
        Some(descriptor_handles)
    }
}

impl<const heap_type: D3D12_DESCRIPTOR_HEAP_TYPE> Drop for TgDescriptorHandle<heap_type> {
    ///dropする際、free_listにindexを返却してからdropする
    fn drop(&mut self) {
        self.descriptor_controller.lock().unwrap().free(self.index);
    }
}

impl<const heap_type: D3D12_DESCRIPTOR_HEAP_TYPE> Default for TgDescriptorHandle<heap_type>{
    fn default() -> Self {
        TgDescriptorHandle{
            cpu_hanle: D3D12_CPU_DESCRIPTOR_HANDLE { ptr: 0 },
            gpu_hanle: D3D12_GPU_DESCRIPTOR_HANDLE { ptr: 0 },
            heap_type: 0,
            dynamic_descriptor_number: 0,
            index: 0,
            descriptor_controller: Arc::new(Mutex::new(TgDescriptorController::new(512)))
        }
    }
}
impl TgDescriptorController {
    ///あとどれくらいHandleを引き出せるかを示す。
    fn free_item_left(&self) -> u32 {
        self.list.iter().map(|node| match node.is_empty{
            true => {node.count}
            false => {0}
        }).sum()
    }
}

impl Default for TgD3d12DescriptorHeapDesc {
    fn default() -> Self {
        TgD3d12DescriptorHeapDesc {
            dynamic_descriptors: 1,
            static_descriptors: 0,
            flags: D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE,
            node_mask: 0,
        }
    }
}