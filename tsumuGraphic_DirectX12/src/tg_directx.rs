use std::ffi::CString;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ptr::null_mut;
use winapi::ctypes::c_void;
use winapi::shared::minwindef::{DWORD, UINT};
use winapi::shared::winerror::S_OK;
use winapi::um::d3d12::{D3D12_CPU_DESCRIPTOR_HANDLE, D3D12_DESCRIPTOR_HEAP_DESC, D3D12_DESCRIPTOR_HEAP_TYPE, D3D12_GPU_DESCRIPTOR_HANDLE, D3D12_GPU_VIRTUAL_ADDRESS, D3D12_PLACED_SUBRESOURCE_FOOTPRINT, D3D12_RANGE, D3D12_RESOURCE_ALIASING_BARRIER, D3D12_RESOURCE_BARRIER, D3D12_RESOURCE_BARRIER_FLAGS, D3D12_RESOURCE_BARRIER_TYPE_ALIASING, D3D12_RESOURCE_BARRIER_TYPE_TRANSITION, D3D12_RESOURCE_BARRIER_TYPE_UAV, D3D12_RESOURCE_BARRIER_u, D3D12_RESOURCE_DESC, D3D12_RESOURCE_TRANSITION_BARRIER, D3D12_RESOURCE_UAV_BARRIER, D3D12_SUBRESOURCE_FOOTPRINT, D3D12_TEXTURE_COPY_LOCATION, D3D12_TEXTURE_COPY_LOCATION_u, D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT, D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX, ID3D12CommandAllocator, ID3D12DescriptorHeap, ID3D12Device, ID3D12Fence, ID3D12PipelineState, ID3D12Resource, ID3D12RootSignature};
use winapi::um::d3dcommon::{D3D_SHADER_MACRO, ID3D10Blob, ID3DBlob, ID3DInclude};
use winapi::um::d3dcompiler::D3DCompileFromFile;
use winapi::um::handleapi::CloseHandle;
use winapi::um::minwinbase::SECURITY_ATTRIBUTES;
use winapi::um::synchapi::{CreateEventW, WaitForSingleObject};
use winapi::um::winnt::{HRESULT, LPCWSTR, HANDLE, LPCSTR};
use tsugumi_windows_library::{BoolInto, wide_char,HRESULTinto};
use crate::tg_device::TgID3D12Device;
use crate::tg_graphics_command_list::CpID3D12GraphicsCommandList;


pub struct TgResourceDesc(D3D12_RESOURCE_DESC);

///リソースを管理する構造体
/// * `interface` - リソースの本体
/// * `bytesize` - リソースのバイトサイズ
/// * `root_parameter_index` - ルートパラメータを参照するときに必要。レンダーターゲットや深度バッファでは不要
/// * `mapvalue` - Mapした値が入っているよ。これを操作することでリソースをいじれるよ
pub struct CpID3D12Resource<T: 'static,S: 'static> {
    pub(crate) interface: *mut ID3D12Resource,
    ///BUFFER_VIEW構造体で要素のサイズを入れるのに必要
    pub(crate) bytesize:u64,
    pub(crate) root_parameter_index:Option<u64>,
    pub(crate) mapvalue:Option<S>,
    pub(crate) _phantom_s:PhantomData<T>
}
pub struct CpID3D12CommandAllocator(pub(crate) *mut ID3D12CommandAllocator);
pub struct CpID3D12Fence {
    pub(crate) interface:  *mut ID3D12Fence,
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

pub struct TgD3d12TextureCopyLocation {
    pub d3d12_texture_copy_location:D3D12_TEXTURE_COPY_LOCATION,
}
impl<T,S> CpID3D12Resource<T, S> {
    ///mapをするよ。D3D12_RANGEは現状使えないので注意。
    pub fn cp_map(mut self, subresource: UINT, pReadRangeOpt: Option<D3D12_RANGE>) -> Result<CpID3D12Resource<T, &'static mut T>, HRESULT> {
        let pReadRange: *const D3D12_RANGE = match pReadRangeOpt {
            Some(v) => { &v }
            None => { null_mut() }
        };
        unsafe {
            let mut _unknownobj = null_mut();
            match self.interface.as_ref().unwrap().Map(subresource, pReadRange, &mut _unknownobj).result() {
                Ok(v) => {
                    match (_unknownobj as *mut T).as_mut() {
                        None => { Err(v) }
                        Some(_obj) => {
                            let tg_resource = CpID3D12Resource{
                                interface: self.interface,
                                bytesize: self.bytesize,
                                root_parameter_index: self.root_parameter_index,
                                mapvalue: Some(_obj),
                                _phantom_s: Default::default()
                            };
                            Ok(tg_resource)
                        }
                    }
                }
                Err(v) => Err(v)
            }
        }
    }
    ///unmapをするよ。D3D12_RANGEは現状使えないので注意。
    pub fn cp_unmap(&mut self, subresource: UINT, pReadRangeOpt: &Option<D3D12_RANGE>) {
        let pReadRange: *const D3D12_RANGE = match pReadRangeOpt {
            Some(v) => { v }
            None => { null_mut() }
        };
        unsafe {
            self.interface.as_ref().unwrap().Unmap(subresource, pReadRange)
        }
        self.mapvalue.take();
    }
    pub fn tg_get_GPU_Virtal_Address(&self)->D3D12_GPU_VIRTUAL_ADDRESS{
        unsafe {self.interface.as_ref().unwrap().GetGPUVirtualAddress()}
    }
    pub fn tg_get_desc(&self)-> D3D12_RESOURCE_DESC{
        unsafe {
            self.interface.as_ref().unwrap().GetDesc()
        }
    }
}

impl<T> CpID3D12Resource<T,&'static mut [T]> {
    pub fn cp_vec_map(self, subresource: UINT, pReadRangeOpt: Option<D3D12_RANGE>,vector:&Vec<T>)-> Result<CpID3D12Resource<T, &'static mut [T]>, HRESULT>{
        self.tg_array_map(subresource, pReadRangeOpt, vector.len())
    }
    pub fn tg_slice_map(self, subresource: UINT, pReadRangeOpt: Option<D3D12_RANGE>, slice:&[T]) -> Result<CpID3D12Resource<T, &'static mut [T]>, HRESULT>{
        self.tg_array_map(subresource, pReadRangeOpt, slice.len())
    }
    pub fn tg_array_map(self, subresource: UINT, pReadRangeOpt: Option<D3D12_RANGE>, length:usize)-> Result<CpID3D12Resource<T, &'static mut [T]>, HRESULT>{
        //todo:リソースの解放をうまいことしたい。RC型とかが使えるか
        let root_parameter_index = self.root_parameter_index.clone();
        let tg_resource =  self.cp_map(subresource, pReadRangeOpt)?;
        let resource = unsafe { std::slice::from_raw_parts_mut(tg_resource.mapvalue.unwrap(), length) };
        let tg_resource = CpID3D12Resource{
            interface: tg_resource.interface,
            bytesize: tg_resource.bytesize,
            root_parameter_index: root_parameter_index,
            mapvalue: Some(resource),
            _phantom_s: Default::default()
        };
        Ok(tg_resource)
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
            self.interface.as_ref().unwrap().GetCompletedValue()
        }
    }
    pub fn cp_is_reach_fance_value(&self) -> bool {
        self.cp_get_completed_value() >= self.fenceval
    }
    pub fn cp_set_event_on_completion(&self, hEvent: &mut CpEventW) -> Result<HRESULT, HRESULT> {
        unsafe {
            self.interface.as_ref().unwrap().SetEventOnCompletion(self.fenceval, hEvent.0).result()
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

/// # コピーしたいテクスチャの情報を記述するよ
/// * `TgD3d12TextureCopyTypeSubresourceIndex` - コピーしたいサブリソースの番号を入れる
/// * `TgD3d12TextureCopyTypePlacedFootprint` - コピーしたいバッファの開始位置と、そのバッファのリソースをどのように解釈するかを示した[D3D12_SUBRESOURCE_FOOTPRINT](https://docs.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_subresource_footprint)構造体を入れる
pub enum TgD3d12TextureCopyType {
    TgD3d12TextureCopyTypeSubresourceIndex { subresource_index:u32},
    TgD3d12TextureCopyTypePlacedFootprint { _offset: u64, footprint: D3D12_SUBRESOURCE_FOOTPRINT}
}
impl TgD3d12TextureCopyLocation {

    /// # テクスチャの情報を記述するよ
    /// * `desc_type` - リソースの種別
    /// * `resource` - リソース本体
    /// ## **<u><span style="color: red; ">注意！</span>リソースはコマンドリストの実行が終わるまで解放しないこと！</u>**
    pub fn new<T: 'static,S: 'static>(desc_type: TgD3d12TextureCopyType, resource:&mut CpID3D12Resource<T, S>) -> TgD3d12TextureCopyLocation {
        match desc_type {
            TgD3d12TextureCopyType::TgD3d12TextureCopyTypeSubresourceIndex { subresource_index } => {
                TgD3d12TextureCopyLocation::tg_subresource_index(subresource_index,resource)
            }
            TgD3d12TextureCopyType::TgD3d12TextureCopyTypePlacedFootprint { _offset, footprint } => {
                TgD3d12TextureCopyLocation::tg_placed_footprint(D3D12_PLACED_SUBRESOURCE_FOOTPRINT{ Offset: _offset, Footprint: footprint}, resource)
            }
        }
    }
    pub fn tg_subresource_index<T: 'static,S: 'static>(SubresourceIndex:u32, resource:&mut CpID3D12Resource<T, S>) ->TgD3d12TextureCopyLocation{
        TgD3d12TextureCopyLocation{
            d3d12_texture_copy_location: D3D12_TEXTURE_COPY_LOCATION {
                pResource: unsafe{ resource.interface.as_mut().unwrap() },
                Type: D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX,
                u: unsafe{
                    std::mem::transmute::<_,D3D12_TEXTURE_COPY_LOCATION_u>([SubresourceIndex;8])
                }
            }
        }
    }

    pub fn tg_placed_footprint<T: 'static,S: 'static>(Footprint: D3D12_PLACED_SUBRESOURCE_FOOTPRINT, resource:&mut CpID3D12Resource<T, S>) ->TgD3d12TextureCopyLocation{
        TgD3d12TextureCopyLocation{
            d3d12_texture_copy_location: D3D12_TEXTURE_COPY_LOCATION {
                pResource: unsafe{ resource.interface.as_mut().unwrap() },
                Type: D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT ,
                u: unsafe{
                    std::mem::transmute::<_,_>(Footprint)
                }
            }
        }
    }

}