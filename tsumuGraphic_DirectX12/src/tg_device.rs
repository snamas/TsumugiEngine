use std::io::Error;
use std::mem::MaybeUninit;
use std::{ptr, slice};
use std::ptr::null_mut;
use std::sync::{Arc, Mutex};
use winapi::Interface;
use winapi::shared::dxgiformat::DXGI_FORMAT_UNKNOWN;
use winapi::shared::dxgitype::DXGI_SAMPLE_DESC;
use winapi::shared::minwindef::UINT;
use winapi::um::d3d12::{D3D12_CLEAR_VALUE, D3D12_COMMAND_LIST_TYPE, D3D12_COMMAND_LIST_TYPE_DIRECT, D3D12_COMMAND_QUEUE_DESC, D3D12_COMMAND_QUEUE_FLAG_NONE, D3D12_COMMAND_QUEUE_PRIORITY_NORMAL, D3D12_CPU_PAGE_PROPERTY_UNKNOWN, D3D12_DESCRIPTOR_HEAP_DESC, D3D12_DESCRIPTOR_HEAP_FLAG_NONE, D3D12_DESCRIPTOR_HEAP_TYPE, D3D12_DESCRIPTOR_HEAP_TYPE_RTV, D3D12_FENCE_FLAGS, D3D12_GRAPHICS_PIPELINE_STATE_DESC, D3D12_HEAP_FLAG_NONE, D3D12_HEAP_FLAGS, D3D12_HEAP_PROPERTIES, D3D12_HEAP_TYPE_UPLOAD, D3D12_INDEX_BUFFER_VIEW, D3D12_MEMORY_POOL_UNKNOWN, D3D12_RENDER_TARGET_VIEW_DESC, D3D12_RESOURCE_DESC, D3D12_RESOURCE_DIMENSION_BUFFER, D3D12_RESOURCE_FLAG_NONE, D3D12_RESOURCE_STATE_GENERIC_READ, D3D12_RESOURCE_STATES, D3D12_TEXTURE_LAYOUT_ROW_MAJOR, D3D12_VERTEX_BUFFER_VIEW, D3D12CreateDevice, ID3D12CommandAllocator, ID3D12CommandQueue, ID3D12DescriptorHeap, ID3D12Device, ID3D12Fence, ID3D12GraphicsCommandList, ID3D12PipelineState, ID3D12Resource, ID3D12RootSignature};
use winapi::um::d3dcommon::D3D_FEATURE_LEVEL_12_1;
use winapi::um::winnt::HRESULT;
use tsugumi_windows_library::HRESULTinto;
use crate::tg_command_dispatcher::CpID3D12CommandDispacher;
use crate::tg_command_queue::CpID3D12CommandQueue;
use crate::tg_descriptor_controller::{TgD3d12CPUDescriptorHandle, TgD3d12GPUDescriptorHandle, TgDescriptorController, TgDescriptorHandle, TgID3D12DescriptorHeap};
use crate::tg_directx::{CpID3D12CommandAllocator,  CpID3D12Fence, CpID3D12PipelineState, CpID3D12Resource, CpID3D12RootSignature, CpID3DBlob};
use crate::tg_graphics_command_list::CpID3D12GraphicsCommandList;

pub struct TgID3D12Device(pub *const ID3D12Device);
impl TgID3D12Device {
    pub fn new() -> TgID3D12Device {
        match TgID3D12Device::cp_d3d12create_device() {
            Ok(v) => return v,
            Err(v) => {
                println!("last OS error: {:?}", Error::last_os_error());
                panic!("{}", v);
            }
        }
    }
    fn cp_d3d12create_device() -> Result<TgID3D12Device, HRESULT> {
        unsafe {
            let mut _unknownobj = null_mut();
            match D3D12CreateDevice(null_mut(), D3D_FEATURE_LEVEL_12_1, &ID3D12Device::uuidof(), &mut _unknownobj).result() {
                Ok(v) => {
                    let mut _id3d12deviceOpt = (_unknownobj as *const ID3D12Device).as_ref();
                    match _id3d12deviceOpt {
                        Some(v) => return Ok(TgID3D12Device(v)),
                        None => return Err(v)
                    };
                }
                Err(v) => return Err(v)
            };
        }
    }
    pub fn cp_create_command_queue(&self, d3d12command_queue_desc_opt: Option<D3D12_COMMAND_QUEUE_DESC>) -> Result<CpID3D12CommandQueue, HRESULT> {
        let d3d12command_queue_desc = match d3d12command_queue_desc_opt {
            Some(v) => { v }
            None => {
                D3D12_COMMAND_QUEUE_DESC {
                    Type: D3D12_COMMAND_LIST_TYPE_DIRECT,
                    Priority: D3D12_COMMAND_QUEUE_PRIORITY_NORMAL as i32,
                    Flags: D3D12_COMMAND_QUEUE_FLAG_NONE,
                    NodeMask: 0,
                }
            }
        };
        unsafe {
            let mut _unknownobj = null_mut();
            match self.0.as_ref().unwrap().CreateCommandQueue(&d3d12command_queue_desc, &ID3D12CommandQueue::uuidof(), &mut _unknownobj).result() {
                Ok(v) => {
                    let mut _id3d12_command_queue = (_unknownobj as *mut ID3D12CommandQueue).as_mut();
                    match _id3d12_command_queue {
                        Some(v) => return Ok(CpID3D12CommandQueue { value: v, type_: d3d12command_queue_desc.Type }),
                        None => return Err(v)
                    };
                }
                Err(v) => return Err(v)
            }
        }
    }
    pub fn cp_create_descriptor_heap(&self, heap_desc: D3D12_DESCRIPTOR_HEAP_DESC) -> Result<TgID3D12DescriptorHeap, HRESULT> {
        let mut _unknownobj = null_mut();
        unsafe {
            match self.0.as_ref().unwrap().CreateDescriptorHeap(&heap_desc, &ID3D12DescriptorHeap::uuidof(), &mut _unknownobj).result() {
                Ok(v) => {
                    match (_unknownobj as *mut ID3D12DescriptorHeap).as_ref() {
                        Some(_id3d12descripterheap) => {
                            let descriptor_number = heap_desc.NumDescriptors;
                            let gpu_descripter_handle =_id3d12descripterheap.GetGPUDescriptorHandleForHeapStart();
                            let cpu_descripter_heap = _id3d12descripterheap.GetCPUDescriptorHandleForHeapStart();
                            let align_size = self.0.as_ref().unwrap().GetDescriptorHandleIncrementSize(heap_desc.Type);
                            return Ok(TgID3D12DescriptorHeap {
                                value: _id3d12descripterheap,
                                desc: heap_desc,
                                align_size: align_size,
                                tg_d3d12cpudescriptor_handle: TgD3d12CPUDescriptorHandle {
                                    value: cpu_descripter_heap,
                                    descriptor_heap_type: heap_desc.Type,
                                    align_size: align_size,
                                },
                                tg_d3d12gpudescriptor_handle: TgD3d12GPUDescriptorHandle {
                                    value: gpu_descripter_handle,
                                    descriptor_heap_type: heap_desc.Type,
                                    align_size: align_size
                                },
                                descriptor_controller: Arc::new(Mutex::new(TgDescriptorController { free_list: (0..descriptor_number).rev().collect::<Vec<_>>() }))
                            }); }
                        None => { return Err(v); }
                    }
                }
                Err(v) => return Err(v)
            }
        }
    }
    pub fn cp_create_command_allocator(&self, type_: D3D12_COMMAND_LIST_TYPE) -> Result<CpID3D12CommandAllocator, HRESULT> {
        unsafe {
            let mut _unknownobj = null_mut();
            match self.0.as_ref().unwrap().CreateCommandAllocator(type_, &ID3D12CommandAllocator::uuidof(), &mut _unknownobj).result() {
                Ok(v) => {
                    match (_unknownobj as *mut ID3D12CommandAllocator).as_mut() {
                        Some(_id3d12command_allocator) => { return Ok(CpID3D12CommandAllocator(_id3d12command_allocator)); }
                        None => { return Err(v); }
                    }
                }
                Err(v) => return Err(v)
            };
        }
    }
    pub fn cp_create_command_allocators<const N: usize>(&self, type_: D3D12_COMMAND_LIST_TYPE)-> Result<[CpID3D12CommandAllocator;N], HRESULT>{
        let mut commandallocators_uninit:MaybeUninit<[CpID3D12CommandAllocator;N]> = unsafe { MaybeUninit::uninit().assume_init() };
        for i in unsafe{ commandallocators_uninit.assume_init_mut()}{
            let mut alloc = self.cp_create_command_allocator(type_)?;
            std::mem::swap(i, &mut alloc);
            std::mem::forget(alloc);
        }
        let commandallocators = unsafe{ commandallocators_uninit.assume_init()};
        Ok(commandallocators)
    }
    pub fn cp_create_command_list(&self, node_mask: UINT, type_: D3D12_COMMAND_LIST_TYPE, command_allocator: &mut CpID3D12CommandAllocator, initial_pypeline_state_opt: &mut Option<ID3D12PipelineState>) -> Result<CpID3D12GraphicsCommandList, HRESULT> {
        let p_initial_state: *mut ID3D12PipelineState = match initial_pypeline_state_opt {
            Some(v) => { v }
            None => { null_mut() }
        };
        unsafe {
            let mut _unknownobj = null_mut();
            match self.0.as_ref().unwrap().CreateCommandList(node_mask, type_, command_allocator.0, p_initial_state, &ID3D12GraphicsCommandList::uuidof(), &mut _unknownobj).result() {
                Ok(v) => {
                    match (_unknownobj as *mut ID3D12GraphicsCommandList).as_mut() {
                        Some(_id3d12graphics_command_list) => {
                            return Ok(CpID3D12GraphicsCommandList(_id3d12graphics_command_list));
                        }
                        None => { return Err(v); }
                    }
                }
                Err(v) => return Err(v)
            }
        }
    }
    pub fn cp_create_command_lists<const N: usize>(&self, node_mask: UINT, type_: D3D12_COMMAND_LIST_TYPE, command_allocator: &mut CpID3D12CommandAllocator, initial_pypeline_state_opt: &mut Option<ID3D12PipelineState>)-> Result<[CpID3D12GraphicsCommandList;N], HRESULT>{
        let mut commandlists_uninit:MaybeUninit<[CpID3D12GraphicsCommandList;N]>= unsafe { MaybeUninit::uninit().assume_init() };
        for i in unsafe{commandlists_uninit.assume_init_mut()}{
            let mut alloc = self.cp_create_command_list(node_mask,type_,command_allocator,initial_pypeline_state_opt)?;
            std::mem::swap(i, &mut alloc);
            std::mem::forget(alloc);
        }
        let commandlists = unsafe{ commandlists_uninit.assume_init()};
        Ok(commandlists)

    }
    pub fn cp_create_command_dispacher<'a>(&self, node_mask: UINT, cp_id3d12command_queue: &'a CpID3D12CommandQueue, listnum: u32, mut p_initial_state_opt: Option<ID3D12PipelineState>) -> Result<CpID3D12CommandDispacher<'a>, HRESULT> {
        let mut _id3d12command_allocator = self.cp_create_command_allocator(cp_id3d12command_queue.type_)?;
        let mut command_lists = (0..listnum).map(|index| -> CpID3D12GraphicsCommandList {
            let commandlist = self.cp_create_command_list(node_mask, cp_id3d12command_queue.type_, &mut _id3d12command_allocator, &mut p_initial_state_opt).unwrap_or_else(|v| { panic!("{}", v) });
            ///最初のリセットに備え、Closeしておく
            commandlist.cp_close();
            commandlist
        }).collect();
        return Ok(CpID3D12CommandDispacher {
            command_queue: &cp_id3d12command_queue,
            command_allocator: _id3d12command_allocator,
            command_lists: command_lists,
        });
    }
    pub fn cp_get_descriptor_handle_increment_size(&self, DescriptorHeapType: D3D12_DESCRIPTOR_HEAP_TYPE) -> UINT {
        unsafe { self.0.as_ref().unwrap().GetDescriptorHandleIncrementSize(DescriptorHeapType) }
    }
    pub fn cp_create_render_target_view<T>(&self, pResource: &mut CpID3D12Resource<T>, pDesc_opt: Option<D3D12_RENDER_TARGET_VIEW_DESC>, DestDescriptor: &TgDescriptorHandle) {
        let pDesc: *const D3D12_RENDER_TARGET_VIEW_DESC = match pDesc_opt {
            Some(v) => { &v }
            None => { null_mut() }
        };
        unsafe { self.0.as_ref().unwrap().CreateRenderTargetView(pResource.value, pDesc, DestDescriptor.cpu_hanle) }
    }
    pub fn cp_create_fence(&self, initialValue: u64, flags: D3D12_FENCE_FLAGS) -> Result<CpID3D12Fence, HRESULT> {
        unsafe {
            let mut _unknownobj = null_mut();
            match self.0.as_ref().unwrap().CreateFence(initialValue, flags, &ID3D12Fence::uuidof(), &mut _unknownobj).result() {
                Ok(v) => {
                    match (_unknownobj as *mut ID3D12Fence).as_mut() {
                        Some(_id3d12_fence) => { return Ok(CpID3D12Fence { value: _id3d12_fence, fenceval: initialValue }); }
                        None => { return Err(v); }
                    }
                }
                Err(v) => return Err(v)
            }
        }
    }
    pub fn cp_create_committed_resource<T>(&self, pHeapProperties: &D3D12_HEAP_PROPERTIES, HeapFlags: D3D12_HEAP_FLAGS, pResourceDesc: &D3D12_RESOURCE_DESC, InitialResourceState: D3D12_RESOURCE_STATES, pOptimizedClearValueOpt: &Option<D3D12_CLEAR_VALUE>, data: &T) -> Result<CpID3D12Resource<T>, HRESULT> {
        let pOptimizedClearValue: *const D3D12_CLEAR_VALUE = match pOptimizedClearValueOpt {
            Some(v) => { v }
            None => { null_mut() }
        };
        unsafe {
            let mut _unknownobj = null_mut();
            match self.0.as_ref().unwrap().CreateCommittedResource(pHeapProperties, HeapFlags, pResourceDesc, InitialResourceState, pOptimizedClearValue, &ID3D12Resource::uuidof(), &mut _unknownobj).result() {
                Ok(v) => {
                    match (_unknownobj as *mut ID3D12Resource).as_mut() {
                        Some(_id3d12_resorce) => {
                            return Ok(CpID3D12Resource { value: _id3d12_resorce, size: std::mem::size_of_val(data) as u32, _phantom: Default::default() });
                        }

                        None => { return Err(v); }
                    }
                }
                Err(v) => return Err(v)
            }
        }
    }
    // pub fn cp_create_buffer_resource<T>(&self, nodemask: u32, vertices: &Vec<T>) -> Result<CpID3D12Resource<Vec<T>>, HRESULT> {
    //     let heapProperties = D3D12_HEAP_PROPERTIES {
    //         Type: D3D12_HEAP_TYPE_UPLOAD,
    //         CPUPageProperty: D3D12_CPU_PAGE_PROPERTY_UNKNOWN,
    //         MemoryPoolPreference: D3D12_MEMORY_POOL_UNKNOWN,
    //         CreationNodeMask: nodemask,
    //         VisibleNodeMask: nodemask,
    //     };
    //     let resourceDesc = D3D12_RESOURCE_DESC {
    //         Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
    //         Alignment: 0,
    //         Width: std::mem::size_of_val(vertices.as_ref()) as u64,
    //         Height: 1,
    //         DepthOrArraySize: 1,
    //         MipLevels: 1,
    //         Format: DXGI_FORMAT_UNKNOWN,
    //         SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
    //         Layout: D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
    //         Flags: D3D12_RESOURCE_FLAG_NONE,
    //     };
    //     let vertexRes = self.cp_create_committed_resource(
    //         &heapProperties,
    //         D3D12_HEAP_FLAG_NONE,
    //         &resourceDesc,
    //         D3D12_RESOURCE_STATE_GENERIC_READ,
    //         &None, vertices)?;
    //     Ok(vertexRes)
    // }
    // pub fn cp_create_index_resource(&self, nodemask: u32, indices: &Vec<u32>) -> Result<CpID3D12Resource<Vec<u32>>, HRESULT> {
    //     let heapProperties = D3D12_HEAP_PROPERTIES {
    //         Type: D3D12_HEAP_TYPE_UPLOAD,
    //         CPUPageProperty: D3D12_CPU_PAGE_PROPERTY_UNKNOWN,
    //         MemoryPoolPreference: D3D12_MEMORY_POOL_UNKNOWN,
    //         CreationNodeMask: nodemask,
    //         VisibleNodeMask: nodemask,
    //     };
    //     let resourceDesc = D3D12_RESOURCE_DESC {
    //         Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
    //         Alignment: 0,
    //         Width: std::mem::size_of_val(indices.as_ref()) as u64,
    //         Height: 1,
    //         DepthOrArraySize: 1,
    //         MipLevels: 1,
    //         Format: DXGI_FORMAT_UNKNOWN,
    //         SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
    //         Layout: D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
    //         Flags: D3D12_RESOURCE_FLAG_NONE,
    //     };
    //     let indexRes = self.cp_create_committed_resource(
    //         &heapProperties,
    //         D3D12_HEAP_FLAG_NONE,
    //         &resourceDesc,
    //         D3D12_RESOURCE_STATE_GENERIC_READ,
    //         &None, indices)?;
    //     Ok(indexRes)
    // }
    pub fn cp_create_root_signature(&self, nodeMask: u32, cpid3dblob: &mut CpID3DBlob) -> Result<CpID3D12RootSignature, HRESULT> {
        unsafe {
            let mut _unknownobj = null_mut();
            match self.0.as_ref().unwrap().CreateRootSignature(nodeMask, cpid3dblob.cp_get_buffer_pointer(), cpid3dblob.cp_get_buffer_size(), &ID3D12RootSignature::uuidof(), &mut _unknownobj).result() {
                Ok(v) => {
                    match (_unknownobj as *mut ID3D12RootSignature).as_mut() {
                        Some(_ID3D12RootSignature) => {
                            return Ok(CpID3D12RootSignature(_ID3D12RootSignature));
                        }

                        None => { return Err(v); }
                    }
                }
                Err(v) => return Err(v)
            }
        }
    }
    pub fn cp_create_graphics_pipeline_state(&self, d3d12_graphics_pipeline_state_desc: &D3D12_GRAPHICS_PIPELINE_STATE_DESC) -> Result<CpID3D12PipelineState, HRESULT> {
        unsafe {
            let mut _unknownobj = null_mut();
            match self.0.as_ref().unwrap().CreateGraphicsPipelineState(d3d12_graphics_pipeline_state_desc, &ID3D12PipelineState::uuidof(), &mut _unknownobj).result() {
                Ok(v) => {
                    match (_unknownobj as *mut ID3D12PipelineState).as_mut() {
                        Some(_ID3D12PipelineState) => {
                            return Ok(CpID3D12PipelineState(_ID3D12PipelineState));
                        }
                        None => { return Err(v); }
                    }
                }
                Err(v) => return Err(v)
            }
        }
    }
}