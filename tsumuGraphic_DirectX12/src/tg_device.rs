use std::io::Error;
use std::mem::MaybeUninit;
use std::{ptr, slice};
use std::fmt::Debug;
use std::ptr::null_mut;
use std::sync::{Arc, Mutex};
use winapi::Interface;
use winapi::shared::dxgiformat::DXGI_FORMAT_UNKNOWN;
use winapi::shared::dxgitype::DXGI_SAMPLE_DESC;
use winapi::shared::minwindef::UINT;
use winapi::um::d3d12::{D3D12_CLEAR_VALUE, D3D12_COMMAND_LIST_TYPE, D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV, D3D12_COMMAND_LIST_TYPE_DIRECT, D3D12_COMMAND_QUEUE_DESC, D3D12_COMMAND_QUEUE_FLAG_NONE, D3D12_COMMAND_QUEUE_PRIORITY_NORMAL, D3D12_CPU_PAGE_PROPERTY_UNKNOWN, D3D12_DESCRIPTOR_HEAP_DESC, D3D12_DESCRIPTOR_HEAP_FLAG_NONE, D3D12_DESCRIPTOR_HEAP_TYPE, D3D12_DESCRIPTOR_HEAP_TYPE_RTV, D3D12_FENCE_FLAGS, D3D12_GRAPHICS_PIPELINE_STATE_DESC, D3D12_HEAP_FLAG_NONE, D3D12_HEAP_FLAGS, D3D12_HEAP_PROPERTIES, D3D12_HEAP_TYPE_UPLOAD, D3D12_INDEX_BUFFER_VIEW, D3D12_MEMORY_POOL_UNKNOWN, D3D12_RENDER_TARGET_VIEW_DESC, D3D12_RESOURCE_DESC, D3D12_RESOURCE_DIMENSION_BUFFER, D3D12_RESOURCE_FLAG_NONE, D3D12_RESOURCE_STATE_GENERIC_READ, D3D12_RESOURCE_STATES, D3D12_TEXTURE_LAYOUT_ROW_MAJOR, D3D12_VERTEX_BUFFER_VIEW, D3D12CreateDevice, D3D_ROOT_SIGNATURE_VERSION, ID3D12CommandAllocator, ID3D12CommandQueue, ID3D12DescriptorHeap, ID3D12Device, ID3D12Fence, ID3D12GraphicsCommandList, ID3D12PipelineState, ID3D12Resource, ID3D12RootSignature, D3D12_CONSTANT_BUFFER_VIEW_DESC};
use winapi::um::d3dcommon::D3D_FEATURE_LEVEL_12_1;
use winapi::um::winnt::HRESULT;
use tsugumi_windows_library::HRESULTinto;
use crate::tg_command_dispatcher::CpID3D12CommandDispacher;
use crate::tg_command_queue::CpID3D12CommandQueue;
use crate::tg_descriptor_controller::{TgD3d12CPUDescriptorHandle, TgD3d12DescriptorHeapDesc, TgD3d12GPUDescriptorHandle, TgDescriptorController, TgDescriptorHandle, TgID3D12DescriptorHeap};
use crate::tg_directx::{CpID3D12CommandAllocator, CpID3D12Fence, CpID3D12PipelineState, CpID3D12Resource, CpID3D12RootSignature, CpID3DBlob};
use crate::tg_graphics_command_list::CpID3D12GraphicsCommandList;
use crate::tg_graphics_pipeline::TgD3d12GraphicsPipeline;
use crate::tg_root_signature::TgD3d12RootSignatureDesc;

#[derive(Clone)]
pub struct TgID3D12Device(pub *const ID3D12Device);

//ID3D12Deviceはスレッドセーフだと書いてあったよ
unsafe impl Send for TgID3D12Device {}

unsafe impl Sync for TgID3D12Device {}

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
    pub(crate) fn cp_create_descriptor_heap<const heap_type: D3D12_DESCRIPTOR_HEAP_TYPE>(&self, tg_heap_desc: TgD3d12DescriptorHeapDesc) -> Result<TgID3D12DescriptorHeap<heap_type>, HRESULT> {
        let mut _unknownobj = null_mut();
        let heap_desc = D3D12_DESCRIPTOR_HEAP_DESC {
            Type: heap_type,
            NumDescriptors: tg_heap_desc.dynamic_descriptors + tg_heap_desc.static_descriptors,
            Flags: tg_heap_desc.flags,
            NodeMask: tg_heap_desc.node_mask,
        };
        unsafe {
            match self.0.as_ref().unwrap().CreateDescriptorHeap(&heap_desc, &ID3D12DescriptorHeap::uuidof(), &mut _unknownobj).result() {
                Ok(v) => {
                    match (_unknownobj as *mut ID3D12DescriptorHeap).as_ref() {
                        Some(_id3d12descripterheap) => {
                            let gpu_descripter_handle = _id3d12descripterheap.GetGPUDescriptorHandleForHeapStart();
                            let cpu_descripter_heap = _id3d12descripterheap.GetCPUDescriptorHandleForHeapStart();
                            let align_size = self.0.as_ref().unwrap().GetDescriptorHandleIncrementSize(heap_desc.Type);
                            return Ok(TgID3D12DescriptorHeap {
                                value: _id3d12descripterheap,
                                dynamic_descriptor_number: tg_heap_desc.dynamic_descriptors,
                                static_descriptor_number: tg_heap_desc.static_descriptors,
                                align_size: align_size,
                                tg_d3d12cpudescriptor_handle: TgD3d12CPUDescriptorHandle {
                                    value: cpu_descripter_heap,
                                    descriptor_heap_type: heap_type,
                                    align_size: align_size,
                                },
                                tg_d3d12gpudescriptor_handle: TgD3d12GPUDescriptorHandle {
                                    value: gpu_descripter_handle,
                                    descriptor_heap_type: heap_type,
                                    align_size: align_size,
                                },
                                descriptor_controller: Arc::new(Mutex::new(TgDescriptorController { dynamic_free_list: (0..tg_heap_desc.dynamic_descriptors).rev().collect::<Vec<_>>(), static_position: 0 })),
                            });
                        }
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
    pub fn cp_create_command_allocators<const N: usize>(&self, type_: D3D12_COMMAND_LIST_TYPE) -> Result<[CpID3D12CommandAllocator; N], HRESULT> {
        let mut commandallocators_uninit: MaybeUninit<[CpID3D12CommandAllocator; N]> = unsafe { MaybeUninit::uninit().assume_init() };
        for i in unsafe { commandallocators_uninit.assume_init_mut() } {
            let mut alloc = self.cp_create_command_allocator(type_)?;
            std::mem::swap(i, &mut alloc);
            std::mem::forget(alloc);
        }
        let commandallocators = unsafe { commandallocators_uninit.assume_init() };
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
    pub fn cp_create_command_lists<const N: usize>(&self, node_mask: UINT, type_: D3D12_COMMAND_LIST_TYPE, command_allocator: &mut [CpID3D12CommandAllocator; N], initial_pypeline_state_opt: &mut Option<ID3D12PipelineState>) -> Result<[CpID3D12GraphicsCommandList; N], HRESULT> {
        let mut commandlists_uninit: MaybeUninit<[CpID3D12GraphicsCommandList; N]> = unsafe { MaybeUninit::uninit().assume_init() };
        for (allocator, list) in command_allocator.iter_mut().zip(unsafe { commandlists_uninit.assume_init_mut() }) {
            let mut spawn_list = self.cp_create_command_list(node_mask, type_, allocator, initial_pypeline_state_opt)?;
            std::mem::swap(list, &mut spawn_list);
            std::mem::forget(spawn_list);
        }
        let commandlists = unsafe { commandlists_uninit.assume_init() };
        Ok(commandlists)
    }
    pub fn cp_get_descriptor_handle_increment_size(&self, DescriptorHeapType: D3D12_DESCRIPTOR_HEAP_TYPE) -> UINT {
        unsafe { self.0.as_ref().unwrap().GetDescriptorHandleIncrementSize(DescriptorHeapType) }
    }
    pub fn cp_create_render_target_view<T, S>(&self, pResource: &mut CpID3D12Resource<T, S>, pDesc_opt: Option<D3D12_RENDER_TARGET_VIEW_DESC>, DestDescriptor: &TgDescriptorHandle<D3D12_DESCRIPTOR_HEAP_TYPE_RTV>) {
        let pDesc: *const D3D12_RENDER_TARGET_VIEW_DESC = match pDesc_opt {
            Some(v) => { &v }
            None => { null_mut() }
        };
        unsafe { self.0.as_ref().unwrap().CreateRenderTargetView(pResource.interface, pDesc, DestDescriptor.cpu_hanle) }
    }
    pub fn cp_create_fence(&self, initialValue: u64, flags: D3D12_FENCE_FLAGS) -> Result<CpID3D12Fence, HRESULT> {
        unsafe {
            let mut _unknownobj = null_mut();
            match self.0.as_ref().unwrap().CreateFence(initialValue, flags, &ID3D12Fence::uuidof(), &mut _unknownobj).result() {
                Ok(v) => {
                    match (_unknownobj as *mut ID3D12Fence).as_mut() {
                        Some(_id3d12_fence) => { return Ok(CpID3D12Fence { interface: _id3d12_fence, fenceval: initialValue }); }
                        None => { return Err(v); }
                    }
                }
                Err(v) => return Err(v)
            }
        }
    }
    pub fn cp_create_committed_resource<T, S>(&self, pHeapProperties: &D3D12_HEAP_PROPERTIES, HeapFlags: D3D12_HEAP_FLAGS, pResourceDesc: &D3D12_RESOURCE_DESC, InitialResourceState: D3D12_RESOURCE_STATES, pOptimizedClearValueOpt: &Option<D3D12_CLEAR_VALUE>, size: u64, RootParameterIndex: Option<u64>) -> Result<CpID3D12Resource<T, S>, HRESULT> {
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
                            return Ok(CpID3D12Resource { interface: _id3d12_resorce, bytesize: size, root_parameter_index: RootParameterIndex, mapvalue: None, _phantom_s: Default::default() });
                        }

                        None => { return Err(v); }
                    }
                }
                Err(v) => return Err(v)
            }
        }
    }
    pub fn tg_create_constant_resource(&self, buffer: &Vec<u8>, heapProperties: D3D12_HEAP_PROPERTIES, RootParameterIndex: u64) -> Result<CpID3D12Resource<u8, &'static mut [u8]>, HRESULT> {
        let allinedSize = buffer.len().div_euclid(256) * 256 + 256;

        let resourceDesc = D3D12_RESOURCE_DESC {
            Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
            Alignment: 0,
            Width: allinedSize as u64,
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            Format: DXGI_FORMAT_UNKNOWN,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Layout: D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
            Flags: D3D12_RESOURCE_FLAG_NONE,
        };
        let buffer_res = self.cp_create_committed_resource(
            &heapProperties,
            D3D12_HEAP_FLAG_NONE,
            &resourceDesc,
            D3D12_RESOURCE_STATE_GENERIC_READ,
            &None, allinedSize as u64, Some(RootParameterIndex))?;
        Ok(buffer_res)
    }
    pub fn cp_create_buffer_resource(&self, nodemask: u32, vertices: &Vec<u8>) -> Result<CpID3D12Resource<u8, &'static mut [u8]>, HRESULT> {
        let size: u64 = (vertices.len() * std::mem::size_of::<u8>()) as u64;
        let heapProperties = D3D12_HEAP_PROPERTIES {
            Type: D3D12_HEAP_TYPE_UPLOAD,
            CPUPageProperty: D3D12_CPU_PAGE_PROPERTY_UNKNOWN,
            MemoryPoolPreference: D3D12_MEMORY_POOL_UNKNOWN,
            CreationNodeMask: nodemask,
            VisibleNodeMask: nodemask,
        };
        let resourceDesc = D3D12_RESOURCE_DESC {
            Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
            Alignment: 0,
            Width: size,
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            Format: DXGI_FORMAT_UNKNOWN,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Layout: D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
            Flags: D3D12_RESOURCE_FLAG_NONE,
        };
        let vertexRes = self.cp_create_committed_resource(&heapProperties, D3D12_HEAP_FLAG_NONE, &resourceDesc, D3D12_RESOURCE_STATE_GENERIC_READ, &None, size, None)?;
        Ok(vertexRes)
    }
    pub fn cp_create_index_resource(&self, nodemask: u32, indices: &Vec<u32>) -> Result<CpID3D12Resource<u32, &'static mut [u32]>, HRESULT> {
        let size: u64 = (indices.len() * std::mem::size_of::<u32>()) as u64;
        let heapProperties = D3D12_HEAP_PROPERTIES {
            Type: D3D12_HEAP_TYPE_UPLOAD,
            CPUPageProperty: D3D12_CPU_PAGE_PROPERTY_UNKNOWN,
            MemoryPoolPreference: D3D12_MEMORY_POOL_UNKNOWN,
            CreationNodeMask: nodemask,
            VisibleNodeMask: nodemask,
        };
        let resourceDesc = D3D12_RESOURCE_DESC {
            Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
            Alignment: 0,
            Width: size,
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            Format: DXGI_FORMAT_UNKNOWN,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Layout: D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
            Flags: D3D12_RESOURCE_FLAG_NONE,
        };
        let indexRes = self.cp_create_committed_resource(&heapProperties, D3D12_HEAP_FLAG_NONE, &resourceDesc, D3D12_RESOURCE_STATE_GENERIC_READ, &None, size, None)?;
        Ok(indexRes)
    }
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
    pub fn tg_serialize_create_root_signature(&self, nodeMask: u32, tg_d3d12root_signature_desc: TgD3d12RootSignatureDesc, version: D3D_ROOT_SIGNATURE_VERSION) -> Result<CpID3D12RootSignature, HRESULT> {
        //todo:ここ雑にBlobの処理をしてるよ
        let result = tg_d3d12root_signature_desc.cp_d3d12serialize_root_signature(version);
        match result {
            Ok(mut cpid3dblob) => self.cp_create_root_signature(nodeMask, &mut cpid3dblob),
            Err(errblob) => Err(errblob.1)
        }
    }
    pub fn cp_create_graphics_pipeline_state(&self, d3d12_graphics_pipeline_state_desc: &mut TgD3d12GraphicsPipeline, tg_rootsig_checker: &CpID3D12RootSignature) -> Result<CpID3D12PipelineState, HRESULT> {
        d3d12_graphics_pipeline_state_desc.0.pRootSignature = tg_rootsig_checker.0;
        unsafe {
            let mut _unknownobj = null_mut();
            match self.0.as_ref().unwrap().CreateGraphicsPipelineState(&d3d12_graphics_pipeline_state_desc.0, &ID3D12PipelineState::uuidof(), &mut _unknownobj).result() {
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
    pub fn tg_create_constant_buffer_view<T, S>(&self, resource: &CpID3D12Resource<T, S>, descriptorhandle: &TgDescriptorHandle<D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV>) {
        let descriptor_handle = descriptorhandle.cpu_hanle;
        let gpuadress = resource.tg_get_GPU_Virtal_Address();
        let view_desc = D3D12_CONSTANT_BUFFER_VIEW_DESC {
            BufferLocation: gpuadress,
            SizeInBytes: resource.bytesize as UINT,
        };
        unsafe {
            self.0.as_ref().unwrap().CreateConstantBufferView(&view_desc, descriptor_handle)
        }
    }
}