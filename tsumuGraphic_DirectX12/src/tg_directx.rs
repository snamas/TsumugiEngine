use std::ffi::CString;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ptr::null_mut;
use winapi::ctypes::c_void;
use winapi::shared::minwindef::UINT;
use winapi::shared::winerror::S_OK;
use winapi::um::d3d12::{D3D12_CPU_DESCRIPTOR_HANDLE, D3D12_DESCRIPTOR_HEAP_DESC, D3D12_DESCRIPTOR_HEAP_TYPE, D3D12_RANGE, ID3D12CommandAllocator, ID3D12DescriptorHeap, ID3D12Device, ID3D12Fence, ID3D12PipelineState, ID3D12Resource, ID3D12RootSignature};
use winapi::um::d3dcommon::{D3D_SHADER_MACRO, ID3D10Blob, ID3DBlob, ID3DInclude};
use winapi::um::d3dcompiler::D3DCompileFromFile;
use winapi::um::winnt::{HRESULT, LPCWSTR, HANDLE, LPCSTR};
use tsugumi_windows_library::{BoolInto, wide_char,HRESULTinto};
use crate::tg_device::CpID3D12Device;

pub struct CpID3D12Resource<T: 'static> {
    pub(crate) value: *mut ID3D12Resource,
    pub(crate) size:u32,
    pub(crate) _phantom:PhantomData<T>
}
pub struct CpID3D12DescriptorHeap {
    pub(crate) value: *const ID3D12DescriptorHeap,
    pub(crate) desc: D3D12_DESCRIPTOR_HEAP_DESC,
}
pub struct CpID3D12CommandAllocator(pub(crate) *mut ID3D12CommandAllocator);
pub struct CpID3D12Fence {
    pub(crate) value:  *mut ID3D12Fence,
    pub(crate) fenceval: u64,
}
pub struct CpID3DBlob(pub *const ID3DBlob);
pub struct CpID3D12RootSignature(pub *mut ID3D12RootSignature);
pub struct CpID3D12PipelineState(pub *mut ID3D12PipelineState);

pub struct CpD3D12_CPU_DESCRIPTOR_HANDLE {
    pub(crate) value: D3D12_CPU_DESCRIPTOR_HANDLE,
    DescriptorHeapType: D3D12_DESCRIPTOR_HEAP_TYPE,
}

impl<T: std::clone::Clone + Debug> CpID3D12Resource<T> {
    pub fn cp_map(&mut self, subresource: UINT, pReadRangeOpt: Option<D3D12_RANGE>) -> Result<Box<&'static mut T>, HRESULT> {
        let pReadRange: *const D3D12_RANGE = match pReadRangeOpt {
            Some(v) => { &v }
            None => { null_mut() }
        };
        unsafe {
            let mut _unknownobj = null_mut();
            match self.value.as_ref().unwrap().Map(subresource, pReadRange, &mut _unknownobj).result() {
                Ok(v) => match (_unknownobj as *mut T).as_mut() {
                    None => { Err(v) }
                    Some(_obj) => {
                        Ok(Box::new(_obj))
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

impl CpD3D12_CPU_DESCRIPTOR_HANDLE {
    pub fn cp_descriptor_handle_increment_ptr(&self, cp_id3d12device: &CpID3D12Device, index: u32) -> CpD3D12_CPU_DESCRIPTOR_HANDLE {
        let mut newHandle: CpD3D12_CPU_DESCRIPTOR_HANDLE = CpD3D12_CPU_DESCRIPTOR_HANDLE {
            value: D3D12_CPU_DESCRIPTOR_HANDLE { ptr: self.value.ptr + (index * cp_id3d12device.cp_get_descriptor_handle_increment_size(self.DescriptorHeapType)) as usize },
            DescriptorHeapType: self.DescriptorHeapType,
        };
        return newHandle;
    }
}