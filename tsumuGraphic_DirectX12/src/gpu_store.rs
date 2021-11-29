use std::collections::HashMap;
use std::ffi::CString;
use std::io::Error;
use std::iter::{Zip};
use std::path::Path;
use std::sync::{Arc, Mutex};
use tsumuObject::Tsumugi3DObject;
use tsumuStockCPU::{Attribute, Color, Joint, Material, Texcoord, TsumugiVertexBinary, Weight};
use crate::tg_device::TgID3D12Device;
use nalgebra;
use winapi::shared::dxgiformat::{DXGI_FORMAT_R16G16_UINT, DXGI_FORMAT_R32_UINT, DXGI_FORMAT_R32G32_FLOAT, DXGI_FORMAT_R32G32B32_FLOAT, DXGI_FORMAT_R32G32B32A32_FLOAT};
use winapi::shared::minwindef::UINT;
use winapi::um::d3d12::{D3D12_APPEND_ALIGNED_ELEMENT, D3D12_INDEX_BUFFER_VIEW, D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA, D3D12_INPUT_ELEMENT_DESC, D3D12_VERTEX_BUFFER_VIEW, D3D_ROOT_SIGNATURE_VERSION_1_0};
use crate::tg_directx::CpID3D12RootSignature;
use crate::tg_graphics_pipeline::TgD3d12GraphicsPipeline;

pub struct TsumuGPUStoreData {
    vertex_view: D3D12_VERTEX_BUFFER_VIEW,
    index_view: D3D12_INDEX_BUFFER_VIEW,
    input_element_desc: Vec<D3D12_INPUT_ELEMENT_DESC>,
}
//まあ大丈夫でしょ
unsafe impl Send for TsumuGPUStoreData {}
unsafe impl Sync for TsumuGPUStoreData {}

///データ層の下にはマテリアル層がある
pub struct FigureDataLayer {
    pub(crate) figure_data: Option<Vec<TsumuGPUStoreData>>,
    pub(crate) material_layer: HashMap<u64, MaterialLayer>,
}

///マテリアル層の下にはオブジェクト層がある
pub struct MaterialLayer {
    material: Material,
    object_layer: HashMap<u64, Tsumugi3DObject>,
}
const POSITION:*const str = "POSITION";
const NORMAL:*const str = "NORMAL";
const TANGENT:*const str = "TANGENT";
const COLOR:*const str = "COLOR";
const TEXCOORD:*const str = "TEXCOORD";
const JOINT:*const str = "JOINT";
const WEIGHT:*const str = "WEIGHT";

impl TsumuGPUStoreData {
    ///3DデータをDirectX12用にロードするよ。TsumugiVertexBinaryが可変参照で必要だけど、これは変わらないよ。
    pub fn load(data: &mut Arc<TsumugiVertexBinary>, tg_id3d12device: &TgID3D12Device) -> Vec<Self> {
        data.vertex.iter().zip(data.index.iter()).zip(data.shader_input_attribute.iter()).map(|((vertex, index), attributes)| {
            let mut CpVertResource = tg_id3d12device.cp_create_buffer_resource(0, vertex).unwrap_or_else(|v| { panic!("last OS error: {:?}", Error::last_os_error()) });
            let mut mapvertdata = CpVertResource.cp_map(0, None).unwrap();
            mapvertdata.copy_from_slice(&vertex);
            CpVertResource.cp_unmap(0, &None);
            let TgVertView: D3D12_VERTEX_BUFFER_VIEW = D3D12_VERTEX_BUFFER_VIEW {
                BufferLocation: CpVertResource.tg_get_GPU_Virtal_Address(),
                SizeInBytes: vertex.len() as UINT,
                StrideInBytes: attributes.1,
            };
            let mut CpIndexResource = tg_id3d12device.cp_create_index_resource(0, index).unwrap_or_else(|v| { panic!("last OS error: {:?}", Error::last_os_error()) });
            let mut mapindexdata = CpIndexResource.cp_map(0, None).unwrap();
            mapindexdata.copy_from_slice(&index);
            CpIndexResource.cp_unmap(0, &None);
            let TgIndexView: D3D12_INDEX_BUFFER_VIEW = D3D12_INDEX_BUFFER_VIEW {
                BufferLocation: CpVertResource.tg_get_GPU_Virtal_Address(),
                SizeInBytes: (index.len() * std::mem::size_of::<u32>()) as UINT,
                Format: DXGI_FORMAT_R32_UINT,
            };
            let mut inputElementDesc: Vec<D3D12_INPUT_ELEMENT_DESC> = Vec::with_capacity(attributes.0.len());
            {
                for attr in &attributes.0 {
                    match attr {
                        Attribute::Position => {
                            inputElementDesc.push(
                                D3D12_INPUT_ELEMENT_DESC {
                                    SemanticName: POSITION.cast(),
                                    SemanticIndex: 0,
                                    Format: DXGI_FORMAT_R32G32B32_FLOAT,
                                    InputSlot: 0,
                                    AlignedByteOffset: D3D12_APPEND_ALIGNED_ELEMENT,
                                    InputSlotClass: D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
                                    InstanceDataStepRate: 0,
                                })
                        }
                        Attribute::Normal => {
                            inputElementDesc.push(
                                D3D12_INPUT_ELEMENT_DESC {
                                    SemanticName: NORMAL.cast(),
                                    SemanticIndex: 0,
                                    Format: DXGI_FORMAT_R32G32B32_FLOAT,
                                    InputSlot: 0,
                                    AlignedByteOffset: D3D12_APPEND_ALIGNED_ELEMENT,
                                    InputSlotClass: D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
                                    InstanceDataStepRate: 0,
                                })
                        }
                        Attribute::Tangent => {
                            inputElementDesc.push(
                                D3D12_INPUT_ELEMENT_DESC {
                                    SemanticName: TANGENT.cast(),
                                    SemanticIndex: 0,
                                    Format: DXGI_FORMAT_R32G32B32A32_FLOAT,
                                    InputSlot: 0,
                                    AlignedByteOffset: D3D12_APPEND_ALIGNED_ELEMENT,
                                    InputSlotClass: D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
                                    InstanceDataStepRate: 0,
                                })
                        }
                        Attribute::Color(Color::RGBA_f32) => {
                            inputElementDesc.push(
                                D3D12_INPUT_ELEMENT_DESC {
                                    SemanticName: COLOR.cast(),
                                    SemanticIndex: 0,
                                    Format: DXGI_FORMAT_R32G32B32A32_FLOAT,
                                    InputSlot: 0,
                                    AlignedByteOffset: D3D12_APPEND_ALIGNED_ELEMENT,
                                    InputSlotClass: D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
                                    InstanceDataStepRate: 0,
                                })
                        }
                        Attribute::Texcoord(Texcoord::f32) => {
                            inputElementDesc.push(
                                D3D12_INPUT_ELEMENT_DESC {
                                    SemanticName: TEXCOORD.cast(),
                                    SemanticIndex: 0,
                                    Format: DXGI_FORMAT_R32G32_FLOAT,
                                    InputSlot: 0,
                                    AlignedByteOffset: D3D12_APPEND_ALIGNED_ELEMENT,
                                    InputSlotClass: D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
                                    InstanceDataStepRate: 0,
                                })
                        }
                        Attribute::Joint(Joint::u16) => {
                            inputElementDesc.push(
                                D3D12_INPUT_ELEMENT_DESC {
                                    SemanticName: JOINT.cast(),
                                    SemanticIndex: 0,
                                    Format: DXGI_FORMAT_R16G16_UINT,
                                    InputSlot: 0,
                                    AlignedByteOffset: D3D12_APPEND_ALIGNED_ELEMENT,
                                    InputSlotClass: D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
                                    InstanceDataStepRate: 0,
                                })
                        }
                        Attribute::Weight(Weight::f32) => {
                            inputElementDesc.push(
                                D3D12_INPUT_ELEMENT_DESC {
                                    SemanticName: WEIGHT.cast(),
                                    SemanticIndex: 0,
                                    Format: DXGI_FORMAT_R32G32B32A32_FLOAT,
                                    InputSlot: 0,
                                    AlignedByteOffset: D3D12_APPEND_ALIGNED_ELEMENT,
                                    InputSlotClass: D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
                                    InstanceDataStepRate: 0,
                                })
                        }
                        _ => { todo!("まだここはできてないよ") }
                    }
                }
            }
            TsumuGPUStoreData {
                vertex_view: TgVertView,
                index_view: TgIndexView,
                input_element_desc: inputElementDesc,
            }
        }).collect()
        // let cp_d3d12_root_signature_desc: CpD3D12_ROOT_SIGNATURE_DESC = Default::default();
        // let rootSigBlob = cp_d3d12_root_signature_desc.cp_d3d12serialize_root_signature(D3D_ROOT_SIGNATURE_VERSION_1_0).unwrap_or_else(|v| { panic!("last OS error: {:?}", Error::last_os_error()) });
        // let mut rootsignature = cp_id3d12device.cp_create_root_signature(0, &rootSigBlob).unwrap_or_else(|v| { panic!("last OS error: {:?}", Error::last_os_error()) });
        // let mut cpgraphicsPipelineStateDesc = CpD3D12_GRAPHICS_PIPELINE_STATE_DESC::create_d3d12_graphics_pipeline_state_desc(&vsBlob, &psBlob, &input_element_desc, &mut rootsignature, None, None, None);
        // let pipelineState = cp_id3d12device.cp_create_graphics_pipeline_state(&cpgraphicsPipelineStateDesc).unwrap_or_else(|v| {
        //     println!("last OS error: {:?}", Error::last_os_error());
        //     panic!("last OS error: {:?}", v)
        // });
    }
}