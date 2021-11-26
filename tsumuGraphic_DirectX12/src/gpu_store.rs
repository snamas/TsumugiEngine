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

struct TsumuGPUStoreElement {
    vertex_view:D3D12_VERTEX_BUFFER_VIEW,
    index_view:D3D12_INDEX_BUFFER_VIEW,
    input_element_desc: Vec<D3D12_INPUT_ELEMENT_DESC>
}
///StoreListは3Dデータを管理する。Pathは3Dデータのパス
struct TsumugiGPUStoreList {
    list: Arc<Mutex<HashMap<&'static Path,FigureDataLayer>>>,
}
///データ層の下にはマテリアル層がある
struct FigureDataLayer {
    figure_data:TsumuGPUStoreElement,
    material_layer:HashMap<u64,MaterialLayer>
}
///マテリアル層の下にはオブジェクト層がある
struct MaterialLayer{
    material:Material,
    object_layer:HashMap<u64, Tsumugi3DObject>
}


impl TsumuGPUStoreElement {
    ///3DデータをDirectX12用にロードするよ。TsumugiVertexBinaryが可変参照で必要だけど、これは変わらないよ。
    fn load(&mut self,data: &mut TsumugiVertexBinary, tg_id3d12device: &TgID3D12Device)->Vec<Self> {
        data.vertex.iter_mut().zip(data.index.iter_mut()).zip(data.shader_input_attribute.iter_mut()).map(|((mut vertex,mut index), mut attributes)|{
            let mut CpVertResource = tg_id3d12device.cp_create_buffer_resource(0, vertex).unwrap_or_else(|v| { panic!("last OS error: {:?}", Error::last_os_error()) });
            let mut mapvertdata = CpVertResource.cp_map(0, None).unwrap();
            mapvertdata.copy_from_slice(&vertex);
            CpVertResource.cp_unmap(0,&None);
            let TgVertView:D3D12_VERTEX_BUFFER_VIEW = D3D12_VERTEX_BUFFER_VIEW{
                BufferLocation: CpVertResource.tg_get_GPU_Virtal_Address(),
                SizeInBytes: vertex.len() as UINT,
                StrideInBytes: attributes.1
            };
            let mut CpIndexResource = tg_id3d12device.cp_create_index_resource(0, index).unwrap_or_else(|v| { panic!("last OS error: {:?}", Error::last_os_error()) });
            let mut mapindexdata = CpIndexResource.cp_map(0, None).unwrap();
            mapindexdata.copy_from_slice(&index);
            CpIndexResource.cp_unmap(0,&None);
            let TgIndexView:D3D12_INDEX_BUFFER_VIEW = D3D12_INDEX_BUFFER_VIEW{
                BufferLocation: CpVertResource.tg_get_GPU_Virtal_Address(),
                SizeInBytes: (index.len() * std::mem::size_of::<u32>()) as UINT,
                Format: DXGI_FORMAT_R32_UINT
            };
            let mut inputElementDesc: Vec<D3D12_INPUT_ELEMENT_DESC> = Vec::with_capacity(attributes.0.len());
            {
                for attr in &mut attributes.0 {
                    match attr {
                        Attribute::Position => {
                            inputElementDesc.push(
                                D3D12_INPUT_ELEMENT_DESC {
                                    SemanticName: CString::new("POSITION").expect("CString::new failed").into_raw(),
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
                                    SemanticName: CString::new("NORMAL").expect("CString::new failed").into_raw(),
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
                                    SemanticName: CString::new("TANGENT").expect("CString::new failed").into_raw(),
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
                                    SemanticName: CString::new("COLOR").expect("CString::new failed").into_raw(),
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
                                    SemanticName: CString::new("TEXCOORD").expect("CString::new failed").into_raw(),
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
                                    SemanticName: CString::new("JOINT").expect("CString::new failed").into_raw(),
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
                                    SemanticName: CString::new("WEIGHT").expect("CString::new failed").into_raw(),
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
            TsumuGPUStoreElement{
                vertex_view: TgVertView,
                index_view: TgIndexView,
                input_element_desc: inputElementDesc
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
fn main() {
    let a1 = [1, 2, 3];
    let a2 = [4, 5, 6];
    let a3 = [7, 8, 9];
    let mut iter = a1.iter().zip(a2.iter()).zip(a3.iter());

}