use std::collections::HashMap;
use std::ffi::CString;
use std::io::Error;
use std::iter::{Zip};
use std::path::Path;
use std::sync::{Arc, Mutex};
use tsumuObject::Tsumugi3DObject;
use tsumuFigureStockCPU::{Attribute, Color, Joint, Texcoord, TsumugiVertexBinary, Weight};
use crate::tg_device::TgID3D12Device;
use nalgebra;
use winapi::shared::dxgiformat::{DXGI_FORMAT_R16G16_UINT, DXGI_FORMAT_R32_UINT, DXGI_FORMAT_R32G32_FLOAT, DXGI_FORMAT_R32G32B32_FLOAT, DXGI_FORMAT_R32G32B32A32_FLOAT};
use winapi::shared::minwindef::UINT;
use winapi::um::d3d12::{D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV, D3D12_APPEND_ALIGNED_ELEMENT, D3D12_INDEX_BUFFER_VIEW, D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA, D3D12_INPUT_ELEMENT_DESC, D3D12_VERTEX_BUFFER_VIEW, D3D_ROOT_SIGNATURE_VERSION_1_0};
use tsumugiShaderStock::TsumugiMaterial;
use tsumugiSimpleHashMap::TsumuHashMap;
use crate::tg_descriptor_controller::TgDescriptorHandle;
use crate::tg_directx::{CpID3D12PipelineState, CpID3D12Resource, CpID3D12RootSignature};
use crate::tg_graphics_pipeline::TgD3d12GraphicsPipeline;

pub struct TsumuGPUFigureDataStore {
    pub vertex_view: D3D12_VERTEX_BUFFER_VIEW,
    pub index_view: D3D12_INDEX_BUFFER_VIEW,
    pub index_len: u32,
}

//まあ大丈夫でしょ
unsafe impl Send for TsumuGPUFigureDataStore {}

unsafe impl Sync for TsumuGPUFigureDataStore {}

///データ層の下にはマテリアル層がある（マテリアルの配列サイズはデータのサイズと基本同じ。同じでない場合は０番が参照される。）
pub struct FigureDataLayer {
    pub figure_data: Option<Vec<TsumuGPUFigureDataStore>>,
    pub material_layer: HashMap<u64, MaterialLayer>,
}

///マテリアル層の下にはオブジェクト層がある
pub struct MaterialLayer {
    // todo:マテリアルが任意に消せるようにしたい。
    pub(crate) material: TsumuHashMap<(CpID3D12PipelineState, CpID3D12RootSignature, MaterialCBV,MaterialDescTable)>,
    pub(crate) object_layer: HashMap<u64, Tsumugi3DObject>,
}

pub struct MaterialCBV(
    pub Vec<(CpID3D12Resource<u8, &'static mut [u8]>, TgDescriptorHandle<D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV>)>
);
unsafe impl Send for MaterialCBV {}
unsafe impl Sync for MaterialCBV {}


pub struct MaterialDescTable(
    pub Vec<(CpID3D12Resource<u8, &'static mut [u8]>, TgDescriptorHandle<D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV>)>
);
unsafe impl Send for MaterialDescTable {}
unsafe impl Sync for MaterialDescTable {}

impl TsumuGPUFigureDataStore {
    ///3DデータをDirectX12用にロードするよ。TsumugiVertexBinaryが可変参照で必要だけど、これは変わらないよ。
    pub fn load(data: &mut TsumugiVertexBinary, tg_id3d12device: &TgID3D12Device) -> Vec<Self> {
        data.vertex.iter().zip(data.index.iter()).zip(data.shader_input_attribute.iter()).map(|((vertex, index), attributes)| {
            let mut CpVertResource = tg_id3d12device.cp_create_buffer_resource_from_vec(0, vertex).unwrap_or_else(|v| { panic!("last OS error: {:?}", Error::last_os_error()) });
            //todo:これDropするときに値が消去されないか心配
            let mut mapvertdata = CpVertResource.cp_vec_map(0, None, vertex).unwrap();
            mapvertdata.mapvalue.as_mut().unwrap().copy_from_slice(&vertex);
            mapvertdata.cp_unmap(0, &None);
            let TgVertView: D3D12_VERTEX_BUFFER_VIEW = D3D12_VERTEX_BUFFER_VIEW {
                BufferLocation: mapvertdata.tg_get_GPU_Virtal_Address(),
                SizeInBytes: vertex.len() as UINT,
                StrideInBytes: attributes.1,
            };
            let mut CpIndexResource = tg_id3d12device.cp_create_index_resource(0, index).unwrap_or_else(|v| { panic!("last OS error: {:?}", Error::last_os_error()) });
            let mut mapindexdata = CpIndexResource.cp_vec_map(0, None, &index).unwrap();
            mapindexdata.mapvalue.as_mut().unwrap().copy_from_slice(&index);
            mapindexdata.cp_unmap(0, &None);
            let TgIndexView: D3D12_INDEX_BUFFER_VIEW = D3D12_INDEX_BUFFER_VIEW {
                BufferLocation: mapindexdata.tg_get_GPU_Virtal_Address(),
                SizeInBytes: (index.len() * std::mem::size_of::<u32>()) as UINT,
                Format: DXGI_FORMAT_R32_UINT,
            };
            TsumuGPUFigureDataStore {
                vertex_view: TgVertView,
                index_view: TgIndexView,
                index_len: index.len() as u32,
            }
        }).collect()
        // let cp_d3d12_root_signature_desc: CpD3d12RootSignatureDesc = Default::default();
        // let rootSigBlob = cp_d3d12_root_signature_desc.cp_d3d12serialize_root_signature(D3D_ROOT_SIGNATURE_VERSION_1_0).unwrap_or_else(|v| { panic!("last OS error: {:?}", Error::last_os_error()) });
        // let mut rootsignature = cp_id3d12device.cp_create_root_signature(0, &rootSigBlob).unwrap_or_else(|v| { panic!("last OS error: {:?}", Error::last_os_error()) });
        // let mut cpgraphicsPipelineStateDesc = CpD3D12_GRAPHICS_PIPELINE_STATE_DESC::create_d3d12_graphics_pipeline_state_desc(&vsBlob, &psBlob, &input_element_desc, &mut rootsignature, None, None, None);
        // let pipelineState = cp_id3d12device.cp_create_graphics_pipeline_state(&cpgraphicsPipelineStateDesc).unwrap_or_else(|v| {
        //     println!("last OS error: {:?}", Error::last_os_error());
        //     panic!("last OS error: {:?}", v)
        // });
    }
}