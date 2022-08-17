use std::borrow::{Borrow, BorrowMut};
use std::ops::Range;
use std::ptr::null_mut;
use winapi::shared::minwindef::UINT;
use winapi::um::d3d12::{D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV,D3D12_DESCRIPTOR_HEAP_TYPE_DSV, D3D12_BOX, D3D12_CPU_DESCRIPTOR_HANDLE, D3D12_GPU_DESCRIPTOR_HANDLE, D3D12_INDEX_BUFFER_VIEW, D3D12_PRIMITIVE_TOPOLOGY, D3D12_RECT, D3D12_RESOURCE_BARRIER, D3D12_VERTEX_BUFFER_VIEW, D3D12_VIEWPORT, ID3D12GraphicsCommandList, ID3D12PipelineState, D3D12_CLEAR_FLAG_DEPTH, D3D12_CLEAR_FLAGS};
use winapi::um::winnt::HRESULT;
use tsugumi_windows_library::{HRESULTinto, vector_Hresult};
use crate::tg_descriptor_controller::{TgDescriptorHandle, TgID3D12DescriptorHeap, TgID3D12DescriptorHeapList};
use crate::tg_directx::{CpID3D12CommandAllocator, CpID3D12PipelineState, CpID3D12Resource, CpID3D12RootSignature, TgD3d12TextureCopyLocation};

pub struct CpID3D12GraphicsCommandList(pub *mut ID3D12GraphicsCommandList);

impl CpID3D12GraphicsCommandList {
    pub fn cp_reset(&self, cp_id3d12command_allocator: &mut CpID3D12CommandAllocator, p_initial_state_opt: &mut Option<ID3D12PipelineState>) -> Result<HRESULT, HRESULT> {
        let p_initial_state: *mut ID3D12PipelineState = match p_initial_state_opt {
            Some(v) => { v }
            None => { null_mut() }
        };
        unsafe {
            return self.0.as_ref().unwrap().Reset(cp_id3d12command_allocator.0, p_initial_state).result();
        }
    }

    pub fn cp_close(&self) -> Result<HRESULT, HRESULT> {
        unsafe {
            return self.0.as_ref().unwrap().Close().result();
        }
    }

    pub fn cp_omset_render_targets(&self, d3d12_cpu_descriptor_handle: &Vec<D3D12_CPU_DESCRIPTOR_HANDLE>, rts_single_handle_to_descriptor_range: bool, p_depth_stencil_descriptor_opt: Option<&D3D12_CPU_DESCRIPTOR_HANDLE>) {
        let p_depth_stencil_descriptor: *const D3D12_CPU_DESCRIPTOR_HANDLE = match p_depth_stencil_descriptor_opt {
            Some(v) => { v }
            None => { null_mut() }
        };
        unsafe {
            return self.0.as_ref().unwrap().OMSetRenderTargets(d3d12_cpu_descriptor_handle.len() as u32, d3d12_cpu_descriptor_handle.as_ptr(), i32::from(rts_single_handle_to_descriptor_range), p_depth_stencil_descriptor);
        }
    }
    ///リソースバリアを張る。
    /// このとき、リストが実行されるまでポインタを保持する必要はない（すぐ解放できる）
    pub fn cp_resource_barrier(&self, d3d12_resource_barrier: &Vec<D3D12_RESOURCE_BARRIER>) {
        unsafe {
            self.0.as_ref().unwrap().ResourceBarrier(d3d12_resource_barrier.len() as u32, d3d12_resource_barrier.as_ptr())
        }
    }

    pub fn cp_iaset_vertex_buffers(&self, StartSlot: UINT, d3d12_vertex_buffer_view: &Vec<D3D12_VERTEX_BUFFER_VIEW>) {
        unsafe {
            self.0.as_ref().unwrap().IASetVertexBuffers(StartSlot, d3d12_vertex_buffer_view.len() as u32, d3d12_vertex_buffer_view.as_ptr())
        }
    }

    pub fn cp_iaset_index_buffer(&self, d3d12_vertex_buffer_view: &D3D12_INDEX_BUFFER_VIEW) {
        unsafe {
            self.0.as_ref().unwrap().IASetIndexBuffer(d3d12_vertex_buffer_view)
        }
    }

    pub fn cp_clear_render_target_view(&self, RenderTargetView: &D3D12_CPU_DESCRIPTOR_HANDLE, ColorRGBA: &[f32; 4], pRects_opt: Option<&Vec<D3D12_RECT>>) {
        let (NumRects, pRects): (u32, *const D3D12_RECT) = match pRects_opt {
            Some(v) => { (v.len() as u32, v.as_ptr()) }
            None => { (0, null_mut()) }
        };
        unsafe {
            self.0.as_ref().unwrap().ClearRenderTargetView(*RenderTargetView, ColorRGBA, NumRects, pRects)
        }
    }

    pub fn tg_clear_depth_stencil_view(&self, descriptorhandle: &TgDescriptorHandle<D3D12_DESCRIPTOR_HEAP_TYPE_DSV>,clear_flag:D3D12_CLEAR_FLAGS, pRects_opt: Option<&Vec<D3D12_RECT>>) {
        let (NumRects, pRects): (u32, *const D3D12_RECT) = match pRects_opt {
            Some(v) => { (v.len() as u32, v.as_ptr()) }
            None => { (0, null_mut()) }
        };
        unsafe {
            self.0.as_ref().unwrap().ClearDepthStencilView(descriptorhandle.cpu_hanle, clear_flag,1.0f32,0, NumRects, pRects)
        }
    }
    pub fn cp_rs_set_viewports(&self, pViewports: &Vec<D3D12_VIEWPORT>) {
        unsafe {
            self.0.as_ref().unwrap().RSSetViewports(pViewports.len() as u32, pViewports.as_ptr())
        }
    }

    pub fn cp_rs_set_scissor_rects(&self, pRects: &Vec<D3D12_RECT>) {
        unsafe {
            self.0.as_ref().unwrap().RSSetScissorRects(pRects.len() as u32, pRects.as_ptr())
        }
    }
    pub fn tg_set_descriptor_heaps(&self, descriptor_heap_list: &mut TgID3D12DescriptorHeapList) {
        unsafe {
            self.0.as_ref().unwrap().SetDescriptorHeaps(2, [descriptor_heap_list.cbv_srv_uav.value,descriptor_heap_list.sampler.value].as_mut_ptr());
        }
    }

    pub fn cp_set_pipeline_states(&self, pPipelineState: &mut CpID3D12PipelineState) {
        unsafe {
            self.0.as_ref().unwrap().SetPipelineState(pPipelineState.0)
        }
    }

    pub fn cp_set_graphics_root_signature(&self, pPipelineState: &mut CpID3D12RootSignature) {
        unsafe {
            self.0.as_ref().unwrap().SetGraphicsRootSignature(pPipelineState.0)
        }
    }


    pub fn tg_set_graphics_root_constant_buffer_view(&self, resource: &CpID3D12Resource<u8, &'static mut [u8]>) {
        unsafe {
            self.0.as_ref().unwrap().SetGraphicsRootConstantBufferView(resource.root_parameter_index.unwrap() as UINT, resource.tg_get_GPU_Virtal_Address())
        }
    }

    pub fn tg_set_graphics_root_descriptor_table(&self, resource: &CpID3D12Resource<u8, &'static mut [u8]>,descriptorHandle:&TgDescriptorHandle<D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV>) {
        unsafe {
            self.0.as_ref().unwrap().SetGraphicsRootDescriptorTable(resource.root_parameter_index.unwrap() as UINT, descriptorHandle.gpu_hanle)
        }
    }
    pub fn cp_iaset_primitive_topology(&self, PrimitiveTopology: D3D12_PRIMITIVE_TOPOLOGY) {
        unsafe {
            self.0.as_ref().unwrap().IASetPrimitiveTopology(PrimitiveTopology)
        }
    }

    pub fn cp_draw_indexed_instanced(&self, IndexCountPerInstance: u32, InstanceCount: u32, StartIndexLocation: u32, BaseVertexLocation: i32, StartInstanceLocation: u32) {
        unsafe {
            self.0.as_ref().unwrap().DrawIndexedInstanced(IndexCountPerInstance, InstanceCount, StartIndexLocation, BaseVertexLocation, StartInstanceLocation)
        }
    }
    /// # テクスチャのコピーを行う命令を発行するよ
    /// * `pDst` - 送り先のテクスチャリソースの情報
    /// * `DstX` - テクスチャの一部分だけ送ることが出来るよ。送りたい左上のX座標を指定
    /// * `DstY` - 左上のY座標を指定
    /// * `DstZ` - 左上のZ座標を指定
    /// * `pSrc` - 送りたいテクスチャの情報
    /// * `pSrcBoxOpt` - 送りたいテクスチャの範囲を[D3D12_BOX](https://docs.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_box)で指定
    /// ## **<u><span style="color: red; ">注意！</span>リソースはこのコマンドリストの実行が終わるまで解放しないこと！</u>**
    pub fn tg_copy_texture_region(&self, pDst: &TgD3d12TextureCopyLocation, DstX: UINT, DstY: UINT, DstZ: UINT, pSrc: &TgD3d12TextureCopyLocation, pSrcBoxOpt: Option<&D3D12_BOX>){
        let pSrcBox: *const D3D12_BOX = match pSrcBoxOpt {
            Some(v) => { v }
            None => { null_mut() }
        };
        unsafe {
            self.0.as_ref().unwrap().CopyTextureRegion(pDst.d3d12_texture_copy_location.borrow(),DstX,DstY,DstZ,pSrc.d3d12_texture_copy_location.borrow(),pSrcBox)
        }
    }
}
pub struct CommandLists(pub Vec<CpID3D12GraphicsCommandList>);
impl  CommandLists {
    pub fn tg_close(&self) -> Result<HRESULT, HRESULT> {
        self.0.iter().map(|command_list|{
            command_list.cp_close()
        }).collect::<Vec<_>>().to_result()
    }
    pub fn tg_reset<const N:usize>(&self,cp_id3d12command_allocator: &mut [CpID3D12CommandAllocator;N], p_initial_state_opt: &mut Option<ID3D12PipelineState>)->Result<HRESULT,HRESULT>{
        self.0.iter().zip(cp_id3d12command_allocator).map(|(commandlist,commandalloc)|{
            commandlist.cp_reset(commandalloc,p_initial_state_opt)
        }).collect::<Vec<_>>().to_result()
    }
    pub fn tg_omset_render_targets(&self,range:Range<usize>,d3d12_cpu_descriptor_handle: &Vec<D3D12_CPU_DESCRIPTOR_HANDLE>, rts_single_handle_to_descriptor_range: bool, p_depth_stencil_descriptor_opt: Option<&D3D12_CPU_DESCRIPTOR_HANDLE>){
        self.0[range].iter().map(|command_list|{
            command_list.cp_omset_render_targets(d3d12_cpu_descriptor_handle,rts_single_handle_to_descriptor_range,p_depth_stencil_descriptor_opt)
        }).collect()
    }
    pub fn tg_set_descriptor_heaps(&self,descriptor_heap_list: &mut TgID3D12DescriptorHeapList){
        self.0.iter().map(|command_list|{
            command_list.tg_set_descriptor_heaps(descriptor_heap_list)
        }).collect()
    }
}