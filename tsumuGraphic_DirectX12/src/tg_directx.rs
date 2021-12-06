use std::ffi::CString;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ptr::null_mut;
use winapi::ctypes::c_void;
use winapi::shared::minwindef::{DWORD, UINT};
use winapi::shared::winerror::S_OK;
use winapi::um::d3d12::{D3D12_CPU_DESCRIPTOR_HANDLE, D3D12_DESCRIPTOR_HEAP_DESC, D3D12_DESCRIPTOR_HEAP_TYPE, D3D12_GPU_DESCRIPTOR_HANDLE, D3D12_GPU_VIRTUAL_ADDRESS, D3D12_RANGE, D3D12_RESOURCE_ALIASING_BARRIER, D3D12_RESOURCE_BARRIER, D3D12_RESOURCE_BARRIER_FLAGS, D3D12_RESOURCE_BARRIER_TYPE_ALIASING, D3D12_RESOURCE_BARRIER_TYPE_TRANSITION, D3D12_RESOURCE_BARRIER_TYPE_UAV, D3D12_RESOURCE_BARRIER_u, D3D12_RESOURCE_TRANSITION_BARRIER, D3D12_RESOURCE_UAV_BARRIER, ID3D12CommandAllocator, ID3D12DescriptorHeap, ID3D12Device, ID3D12Fence, ID3D12PipelineState, ID3D12Resource, ID3D12RootSignature};
use winapi::um::d3dcommon::{D3D_SHADER_MACRO, ID3D10Blob, ID3DBlob, ID3DInclude};
use winapi::um::d3dcompiler::D3DCompileFromFile;
use winapi::um::handleapi::CloseHandle;
use winapi::um::minwinbase::SECURITY_ATTRIBUTES;
use winapi::um::synchapi::{CreateEventW, WaitForSingleObject};
use winapi::um::winnt::{HRESULT, LPCWSTR, HANDLE, LPCSTR};
use tsugumi_windows_library::{BoolInto, wide_char,HRESULTinto};
use crate::tg_device::TgID3D12Device;

pub struct CpID3D12Resource<T: 'static> {
    pub(crate) value: *mut ID3D12Resource,
    ///BUFFER_VIEW構造体で要素のサイズを入れるのに必要
    pub(crate) bytesize:u64,
    pub(crate) _phantom:PhantomData<T>
}
pub struct CpID3D12CommandAllocator(pub(crate) *mut ID3D12CommandAllocator);
pub struct CpID3D12Fence {
    pub(crate) value:  *mut ID3D12Fence,
    pub(crate) fenceval: u64,
}
pub struct CpID3DBlob(pub *const ID3DBlob);
pub struct CpID3D12RootSignature(pub *mut ID3D12RootSignature);
//まあ大丈夫でしょ
unsafe impl Send for CpID3D12RootSignature {}
unsafe impl Sync for CpID3D12RootSignature {}
pub struct CpID3D12PipelineState(pub *mut ID3D12PipelineState);
//まあ大丈夫でしょ
unsafe impl Send for CpID3D12PipelineState {}
unsafe impl Sync for CpID3D12PipelineState {}
pub struct CpD3D12_RESOURCE_BARRIER(pub *mut D3D12_RESOURCE_BARRIER);
pub struct CpEventW(*mut c_void);

impl<T: std::clone::Clone + Debug> CpID3D12Resource<T> {
    pub fn cp_map<S>(&self, subresource: UINT, pReadRangeOpt: Option<D3D12_RANGE>) -> Result<&'static mut S, HRESULT> {
        let pReadRange: *const D3D12_RANGE = match pReadRangeOpt {
            Some(v) => { &v }
            None => { null_mut() }
        };
        unsafe {
            let mut _unknownobj = null_mut();
            match self.value.as_ref().unwrap().Map(subresource, pReadRange, &mut _unknownobj).result() {
                Ok(v) => match (_unknownobj as *mut S).as_mut() {
                    None => { Err(v) }
                    Some(_obj) => {
                        Ok(_obj)
                    }
                }
                Err(v) => Err(v)
            }
        }
    }
    pub fn cp_unmap(&self, subresource: UINT, pReadRangeOpt: &Option<D3D12_RANGE>) {
        let pReadRange: *const D3D12_RANGE = match pReadRangeOpt {
            Some(v) => { v }
            None => { null_mut() }
        };
        unsafe {
            self.value.as_ref().unwrap().Unmap(subresource, pReadRange)
        }
    }
    pub fn cp_slice_map(&self, subresource: UINT, pReadRangeOpt: Option<D3D12_RANGE>, len: impl std::iter::ExactSizeIterator) -> Result<&'static mut [T], HRESULT> {
        let _arr_obj = self.cp_map(subresource, pReadRangeOpt)?;
        let _arr = unsafe { std::slice::from_raw_parts_mut(_arr_obj, len.len()) };
        Ok(_arr)
    }
    pub fn tg_get_GPU_Virtal_Address(&self)->D3D12_GPU_VIRTUAL_ADDRESS{
        unsafe {self.value.as_ref().unwrap().GetGPUVirtualAddress()}
    }
}

impl<T: std::clone::Clone + Debug> CpID3D12Resource<Vec<T>> {
    pub fn cp_vec_map(&self, subresource: UINT, pReadRangeOpt: Option<D3D12_RANGE>,vector:&Vec<T>)-> Result<&'static mut [T], HRESULT>{
        let resource = self.cp_map::<T>(subresource,pReadRangeOpt)?;
        let resource = unsafe { std::slice::from_raw_parts_mut(resource, vector.len()) };
        Ok(resource)
    }
}
impl CpID3DBlob {
    pub fn cp_d3dcompile_from_file(pFileName: &str, pDefinesOpt: Option<&D3D_SHADER_MACRO>, pInclude: *mut ID3DInclude, pEntrypoint: &str, pTarget: &str, Flags1: UINT, Flags2: UINT) -> Result<CpID3DBlob, (CpID3DBlob, HRESULT)> {
        let mut okBlob: *mut ID3D10Blob = null_mut();
        let mut errBlob: *mut ID3D10Blob = null_mut();
        let pDefines: *const D3D_SHADER_MACRO = match pDefinesOpt {
            Some(v) => { v }
            None => { null_mut() }
        };
        let CstrpEntrypoint = CString::new(pEntrypoint).expect("CString::new failed");
        let CstrpTarget = CString::new(pTarget).expect("CString::new failed");

        unsafe {
            match D3DCompileFromFile(pFileName.to_wide_chars().as_ptr(), pDefines, pInclude, CstrpEntrypoint.as_ptr(), CstrpTarget.as_ptr(), Flags1, Flags2, &mut okBlob, &mut errBlob).result() {
                Ok(_) => {
                    Ok(CpID3DBlob(okBlob.as_mut().unwrap()))
                }
                Err(v) => {
                    Err((CpID3DBlob(errBlob.as_mut().unwrap()), v))
                }
            }
        }
    }
    pub fn cp_get_buffer_pointer(&mut self) -> &mut c_void {
        unsafe {
            self.0.as_ref().unwrap().GetBufferPointer().as_mut().unwrap()
        }
    }
    pub fn cp_get_buffer_size(&self) -> usize {
        unsafe {
            self.0.as_ref().unwrap().GetBufferSize()
        }
    }
}

impl CpID3D12CommandAllocator {
    pub fn cp_reset(&self) -> Result<HRESULT, HRESULT> {
        unsafe {
            return self.0.as_ref().unwrap().Reset().result();
        }
    }
}

pub enum CpD3d12ResourceBarrierDescType {
    CpD3d12ResourceTransitionBarrier { d3d12_resource_transition_barrier: D3D12_RESOURCE_TRANSITION_BARRIER, flags: D3D12_RESOURCE_BARRIER_FLAGS },
    CpD3d12ResourceAliasingBarrier { d3d12_resource_aliasing_barrier: D3D12_RESOURCE_ALIASING_BARRIER, flags: D3D12_RESOURCE_BARRIER_FLAGS },
    CpD3D12_RESOURCE_UAV_BARRIER { d3d12_resource_uav_barrier: D3D12_RESOURCE_UAV_BARRIER, flags: D3D12_RESOURCE_BARRIER_FLAGS },
}
impl CpD3D12_RESOURCE_BARRIER {
    ///リソースバリアのタイプで返すD3D12_RESOURCE_BARRIER構造体の共用体部分を決める
    pub fn new(desc_type: CpD3d12ResourceBarrierDescType) -> D3D12_RESOURCE_BARRIER {
        match desc_type {
            CpD3d12ResourceBarrierDescType::CpD3d12ResourceTransitionBarrier { d3d12_resource_transition_barrier, flags } => {
                CpD3D12_RESOURCE_BARRIER::cp_transition(&d3d12_resource_transition_barrier, flags)
            }
            CpD3d12ResourceBarrierDescType::CpD3d12ResourceAliasingBarrier { d3d12_resource_aliasing_barrier, flags } => {
                CpD3D12_RESOURCE_BARRIER::cp_aliasing(&d3d12_resource_aliasing_barrier, flags)
            }
            CpD3d12ResourceBarrierDescType::CpD3D12_RESOURCE_UAV_BARRIER { d3d12_resource_uav_barrier, flags } => {
                CpD3D12_RESOURCE_BARRIER::cp_uav(&d3d12_resource_uav_barrier, flags)
            }
        }
    }
    pub fn cp_transition(d3d12_resource_transition_barrier: &D3D12_RESOURCE_TRANSITION_BARRIER, flags: D3D12_RESOURCE_BARRIER_FLAGS) -> D3D12_RESOURCE_BARRIER {
        D3D12_RESOURCE_BARRIER
        {
            Type: D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
            Flags: flags,
            u: unsafe {
                *std::mem::transmute::<&D3D12_RESOURCE_TRANSITION_BARRIER, &D3D12_RESOURCE_BARRIER_u>(d3d12_resource_transition_barrier)
            },
        }
    }
    pub fn cp_aliasing(d3d12_resource_aliasing_barrier: &D3D12_RESOURCE_ALIASING_BARRIER, flags: D3D12_RESOURCE_BARRIER_FLAGS) -> D3D12_RESOURCE_BARRIER {
        D3D12_RESOURCE_BARRIER
        {
            Type: D3D12_RESOURCE_BARRIER_TYPE_ALIASING,
            Flags: flags,
            u: unsafe {
                *std::mem::transmute::<&D3D12_RESOURCE_ALIASING_BARRIER, &D3D12_RESOURCE_BARRIER_u>(d3d12_resource_aliasing_barrier)
            },
        }
    }
    pub fn cp_uav(d3d12_resource_uav_barrier: &D3D12_RESOURCE_UAV_BARRIER, flags: D3D12_RESOURCE_BARRIER_FLAGS) -> D3D12_RESOURCE_BARRIER {
        D3D12_RESOURCE_BARRIER
        {
            Type: D3D12_RESOURCE_BARRIER_TYPE_UAV,
            Flags: flags,
            u: unsafe {
                *std::mem::transmute::<&D3D12_RESOURCE_UAV_BARRIER, &D3D12_RESOURCE_BARRIER_u>(d3d12_resource_uav_barrier)
            },
        }
    }
}

impl CpID3D12Fence {
    pub fn cp_get_completed_value(&self) -> u64 {
        unsafe {
            self.value.as_ref().unwrap().GetCompletedValue()
        }
    }
    pub fn cp_is_reach_fance_value(&self) -> bool {
        self.cp_get_completed_value() >= self.fenceval
    }
    pub fn cp_set_event_on_completion(&self, hEvent: &mut CpEventW) -> Result<HRESULT, HRESULT> {
        unsafe {
            self.value.as_ref().unwrap().SetEventOnCompletion(self.fenceval, hEvent.0).result()
        }
    }
    pub fn cp_increment_counter(&mut self, incrementvalue: u64) {
        self.fenceval = self.fenceval.wrapping_add(incrementvalue);
    }
}

impl CpEventW{
    pub fn cp_create_event_w(lpEventAttributes_opt: Option<&mut SECURITY_ATTRIBUTES>, bManualReset: bool, bInitialState: bool, lpName_opt: Option<&str>) -> Option<CpEventW> {
        let lpEventAttributes: *mut SECURITY_ATTRIBUTES = match lpEventAttributes_opt {
            Some(v) => { v }
            None => { null_mut() }
        };
        let lpName = match lpName_opt {
            Some(v) => { v.to_wide_chars().as_ptr() }
            None => { null_mut() }
        };
        Some(CpEventW(unsafe { CreateEventW(lpEventAttributes, i32::from(bManualReset), i32::from(bInitialState), lpName).as_mut()? }))
    }
    pub fn cp_wait_for_single_object(&mut self, dwMilliseconds: DWORD) -> DWORD {
        unsafe { WaitForSingleObject(self.0, dwMilliseconds) }
    }
    pub fn cp_CloseHandlet(&mut self) -> bool {
        unsafe { CloseHandle(self.0).intobool() }
    }
}