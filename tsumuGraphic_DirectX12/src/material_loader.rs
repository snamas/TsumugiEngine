use std::collections::HashMap;
use std::ffi::CString;
use std::fs::File;
use std::io;
use std::io::{Error, Read};
use std::path::Path;
use std::sync::{Arc, Mutex};
use winapi::shared::dxgiformat::{DXGI_FORMAT_R16G16_UINT, DXGI_FORMAT_R32G32_FLOAT, DXGI_FORMAT_R32G32B32_FLOAT, DXGI_FORMAT_R32G32B32A32_FLOAT};
use winapi::shared::minwindef::UINT;
use winapi::um::d3d12::{D3D12_APPEND_ALIGNED_ELEMENT, D3D12_DESCRIPTOR_RANGE, D3D12_DESCRIPTOR_RANGE_TYPE_SRV, D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA, D3D12_INPUT_ELEMENT_DESC, D3D12_INPUT_LAYOUT_DESC, D3D12_ROOT_PARAMETER, D3D12_STATIC_SAMPLER_DESC, D3D_ROOT_SIGNATURE_VERSION_1_0};
use winapi::um::winnt::HRESULT;
use tsumuFigureStockCPU::{Attribute, Color, Joint, Texcoord, Weight};
use tsumugiShaderStock::{Material, TsumugiMaterial, TsumugiShader};
use crate::tg_directx::{CpID3D12PipelineState, CpID3D12RootSignature};
use crate::tg_graphics_pipeline::TgD3d12GraphicsPipeline;
use crate::tg_root_signature::TgD3d12RootSignatureDesc;
use crate::tg_sampler::TgStaticSamplerDesc;
use crate::TgID3D12Device;

const POSITION: *const str = "POSITION";
const NORMAL: *const str = "NORMAL";
const TANGENT: *const str = "TANGENT";
const COLOR: *const str = "COLOR";
const TEXCOORD: *const str = "TEXCOORD";
const JOINT: *const str = "JOINT";
const WEIGHT: *const str = "WEIGHT";

pub(crate) trait MaterialLoadDirectx12 {
    fn load(&self, tg_device: &Arc<TgID3D12Device>) -> Vec<(CpID3D12PipelineState,CpID3D12RootSignature)>;
    fn trans_input_elements(attributes: &Vec<Attribute>) -> Vec<D3D12_INPUT_ELEMENT_DESC>;
}

impl MaterialLoadDirectx12 for TsumugiMaterial {
    fn load(&self, tg_device: &Arc<TgID3D12Device>)->Vec<(CpID3D12PipelineState,CpID3D12RootSignature)> {
        self.material.iter().map(|material|{
            let constant_buffer_len = material.buffer.len();
            let discriptor_range: Vec<D3D12_DESCRIPTOR_RANGE> = vec![
                D3D12_DESCRIPTOR_RANGE {
                    RangeType: D3D12_DESCRIPTOR_RANGE_TYPE_SRV,
                    NumDescriptors: material.texture.len() as UINT,
                    BaseShaderRegister: 0,
                    RegisterSpace: 0,
                    OffsetInDescriptorsFromTableStart: 0,
                }];
            let mut sampler: Vec<D3D12_STATIC_SAMPLER_DESC> = Vec::with_capacity(material.texture.len());
            for index in 0..material.texture.len() {
                sampler.push(TgStaticSamplerDesc::default().shader_register(index as u32).0);
            }
            //todo:ここテクスチャが一枚も入って無くても１確保される
            let root_parameter: Vec<D3D12_ROOT_PARAMETER> = Vec::with_capacity(constant_buffer_len + 1);
            //todo:シグネチャーあとで作るよ

            let tg_d3d12_root_signature_desc: TgD3d12RootSignatureDesc = Default::default();
            let root_sig = tg_d3d12_root_signature_desc.cp_d3d12serialize_root_signature(D3D_ROOT_SIGNATURE_VERSION_1_0).and_then(|mut serialized_root_sig| {
                Ok(tg_device.cp_create_root_signature(0, &mut serialized_root_sig).unwrap())
            });
            let input = Self::trans_input_elements(&material.attributes);
            //todo:ここマテリアルの属性をいろいろ入れたいね（現在シェーダー入れるだけ）
            let mut tg_graphics_pipeline_state_desc = TgD3d12GraphicsPipeline::default()
                .vertex_shader(&self.shader_path_vs)
                .pixel_shader(&self.shader_path_ps)
                .input_layout(Self::trans_input_elements(&material.attributes));
            let inputElementDesc = vec![
                D3D12_INPUT_ELEMENT_DESC {
                    SemanticName: CString::new("POSITION").expect("CString::new failed").into_raw(),
                    SemanticIndex: 0,
                    Format: DXGI_FORMAT_R32G32B32_FLOAT,
                    InputSlot: 0,
                    AlignedByteOffset: D3D12_APPEND_ALIGNED_ELEMENT,
                    InputSlotClass: D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
                    InstanceDataStepRate: 0,
                }
            ];
            tg_graphics_pipeline_state_desc.0.InputLayout = D3D12_INPUT_LAYOUT_DESC { pInputElementDescs: input.as_ptr(), NumElements: input.len() as u32 };
            if let Some(gs) = &self.shader_path_gs {
                tg_graphics_pipeline_state_desc = tg_graphics_pipeline_state_desc.geometry_shader(gs);
            }
            if let Some(hs) = &self.shader_path_hs {
                tg_graphics_pipeline_state_desc = tg_graphics_pipeline_state_desc.hull_shader(hs);
            }
            if let Some(ds) = &self.shader_path_ds {
                tg_graphics_pipeline_state_desc = tg_graphics_pipeline_state_desc.domain_shader(ds);
            }
            (tg_device.cp_create_graphics_pipeline_state(&mut tg_graphics_pipeline_state_desc,&root_sig.as_ref().ok().unwrap()).unwrap(),root_sig.ok().unwrap())
        }).collect()
    }

    fn trans_input_elements(attributes: &Vec<Attribute>) -> Vec<D3D12_INPUT_ELEMENT_DESC> {
        let mut inputElement: Vec<D3D12_INPUT_ELEMENT_DESC> = Vec::with_capacity(attributes.len());
        for attr in attributes {
            match attr {
                Attribute::Position => {
                    inputElement.push(
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
                    inputElement.push(
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
                    inputElement.push(
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
                    inputElement.push(
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
                    inputElement.push(
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
                    inputElement.push(
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
                    inputElement.push(
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
        inputElement
    }
}