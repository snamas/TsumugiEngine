use std::collections::HashMap;
use std::ffi::CString;
use std::fs::File;
use std::io;
use std::io::{Error, Read};
use std::path::Path;
use std::sync::{Arc, Mutex};
use winapi::shared::dxgiformat::{DXGI_FORMAT_R16G16_UINT, DXGI_FORMAT_R32G32_FLOAT, DXGI_FORMAT_R32G32B32_FLOAT, DXGI_FORMAT_R32G32B32A32_FLOAT};
use winapi::shared::minwindef::UINT;
use winapi::um::d3d12::{D3D12_APPEND_ALIGNED_ELEMENT, D3D12_CPU_PAGE_PROPERTY_UNKNOWN, D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV, D3D12_DESCRIPTOR_RANGE, D3D12_DESCRIPTOR_RANGE_TYPE_SRV, D3D12_HEAP_PROPERTIES, D3D12_HEAP_TYPE_UPLOAD, D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA, D3D12_INPUT_ELEMENT_DESC, D3D12_INPUT_LAYOUT_DESC, D3D12_MEMORY_POOL_UNKNOWN, D3D12_ROOT_DESCRIPTOR, D3D12_ROOT_PARAMETER, D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT, D3D12_SHADER_VISIBILITY_ALL, D3D12_STATIC_SAMPLER_DESC, D3D12_TEX2D_SRV, D3D_ROOT_SIGNATURE_VERSION_1_0};
use winapi::um::winnt::HRESULT;
use tsumuFigureStockCPU::{Attribute, Color, Joint, Texcoord, Weight};
use tsumugiShaderStock::{Material, TsumugiMaterial, TsumugiShader};
use crate::gpu_figure_store::{MaterialCBV, MaterialDescTable};
use crate::tg_buffer_uploader::BufferUploadBatch;
use crate::tg_descriptor_controller::{TgDescriptorHandle, TgID3D12DescriptorHeapList};
use crate::tg_directx::{CpID3D12PipelineState, CpID3D12Resource, CpID3D12RootSignature};
use crate::tg_graphics_pipeline::TgD3d12GraphicsPipeline;
use crate::tg_root_signature::{TgD3d12RootParameters, TgD3d12RootSignatureDesc};
use crate::tg_sampler::TgStaticSamplerDesc;
use crate::{CpID3D12CommandQueue, TgID3D12Device};
use crate::tg_device::{D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING, TgSrvDimension};

const POSITION: *const str = "POSITION\0";
const NORMAL: *const str = "NORMAL\0";
const TANGENT: *const str = "TANGENT\0";
const COLOR: *const str = "COLOR\0";
const TEXCOORD: *const str = "TEXCOORD\0";
const JOINT: *const str = "JOINT\0";
const WEIGHT: *const str = "WEIGHT\0";

pub(crate) trait MaterialLoadDirectx12 {
    fn load(&self, tg_device: &Arc<TgID3D12Device>,tg_commandqueue: &Arc<Mutex<CpID3D12CommandQueue>>,tg_descriptor_heap:&mut TgID3D12DescriptorHeapList) -> (CpID3D12PipelineState, CpID3D12RootSignature, MaterialCBV,MaterialDescTable);
    fn trans_input_elements(attributes: &Vec<Attribute>) -> Vec<D3D12_INPUT_ELEMENT_DESC>;
}

impl MaterialLoadDirectx12 for TsumugiMaterial {
    fn load(&self, tg_device: &Arc<TgID3D12Device>,tg_commandqueue: &Arc<Mutex<CpID3D12CommandQueue>>, tg_descriptor_heap_list: &mut TgID3D12DescriptorHeapList) ->(CpID3D12PipelineState, CpID3D12RootSignature, MaterialCBV,MaterialDescTable) {
        {
            let constant_buffer_len = self.material.buffer.len();
            let mut root_parameter:TgD3d12RootParameters = TgD3d12RootParameters::with_capacity(constant_buffer_len + 1);
            let resources_cbv = self.material.buffer.iter().enumerate().map(|(i,buffer)|{
                let descriptor_handle = tg_descriptor_heap_list.cbv_srv_uav.allocate_dynamic_descriptor_handle().unwrap_or_else(||{panic!("cbv allocate Failed")});
                let heapProperties = D3D12_HEAP_PROPERTIES {
                    Type: D3D12_HEAP_TYPE_UPLOAD,
                    CPUPageProperty: D3D12_CPU_PAGE_PROPERTY_UNKNOWN,
                    MemoryPoolPreference: D3D12_MEMORY_POOL_UNKNOWN,
                    CreationNodeMask: 0,
                    VisibleNodeMask: 0,
                };
                //todo:ここでunwrapを使用しているので注意。例外をResultで返す機構が必要かも
                let mut resource = tg_device.tg_create_constant_resource(buffer, heapProperties, (root_parameter.0.len()) as u64).unwrap().cp_vec_map(0, None, buffer).unwrap();
                resource.mapvalue.as_mut().unwrap().copy_from_slice(buffer);
                resource.cp_unmap(0,&None);
                tg_device.tg_create_constant_buffer_view(&resource,&descriptor_handle);
                {
                    root_parameter.append_descriptor_cbv(D3D12_ROOT_DESCRIPTOR { ShaderRegister: i as UINT, RegisterSpace: 0 }, D3D12_SHADER_VISIBILITY_ALL);
                }
                (resource,descriptor_handle)
            }).collect::<Vec<(CpID3D12Resource<u8, &'static mut [u8]>,TgDescriptorHandle<D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV>)>>();


            //todo:サンプラーはテクスチャの数確保しなくていい。
            let mut sampler: Vec<D3D12_STATIC_SAMPLER_DESC> = Vec::with_capacity(self.material.texture.len());
            for index in 0..self.material.texture.len() {
                sampler.push(TgStaticSamplerDesc::default().shader_register(index as u32).0);
            }

            ///テクスチャのリソースを作成する。
            let mut batch = BufferUploadBatch::create_batch(tg_device);
            let mut rangevec = Vec::new();
            let resource_textures = self.material.texture.iter().enumerate().map(|(i,path)|{
                let mut texture_resource = tg_device.tg_create_texture_from_file(0, path, &mut batch,(root_parameter.0.len()) as u64);
                //todo:将来的にdescriptorhandleをdescriptortableを一つにすることで、一つにまとめる。
                let descriptor_handle = tg_descriptor_heap_list.cbv_srv_uav.allocate_dynamic_descriptor_handle().unwrap_or_else(||{panic!("cbv allocate Failed")});
                let texDesc  = texture_resource.tg_get_desc();
                tg_device.tg_create_shader_resource_view(&mut texture_resource, texDesc.Format, D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING(), TgSrvDimension::TgTex2dSrv { tex2d_srv: D3D12_TEX2D_SRV {
                    MostDetailedMip: 0,
                    MipLevels: texDesc.MipLevels as u32,
                    PlaneSlice: 0,
                    ResourceMinLODClamp: 0.0f32
                } }, &descriptor_handle);

                {
                    let ranges = vec![D3D12_DESCRIPTOR_RANGE {
                        RangeType: D3D12_DESCRIPTOR_RANGE_TYPE_SRV,
                        NumDescriptors: 1,
                        BaseShaderRegister:  i as UINT,
                        RegisterSpace: 0,
                        OffsetInDescriptorsFromTableStart: 0
                    }];
                    root_parameter.append_descriptor_table(&ranges, D3D12_SHADER_VISIBILITY_ALL);
                    rangevec.push(ranges);
                }
                (texture_resource,descriptor_handle)
            }).collect::<Vec<_>>();

            batch.end(&*tg_commandqueue.lock().unwrap());

            let root_sig_desc = TgD3d12RootSignatureDesc::default()
                .root_parameter(&root_parameter)
                .flag(D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT)
                .static_sampler(&sampler);
            let root_sig = tg_device.tg_serialize_create_root_signature(0, root_sig_desc, D3D_ROOT_SIGNATURE_VERSION_1_0).unwrap();

            let input = Self::trans_input_elements(&self.material.attributes);
            //todo:ここマテリアルの属性をいろいろ入れたいね（現在シェーダー入れるだけ）
            let mut tg_graphics_pipeline_state_desc = TgD3d12GraphicsPipeline::default()
                .vertex_shader(&self.shader_path_vs)
                .pixel_shader(&self.shader_path_ps)
                .input_layout(Self::trans_input_elements(&self.material.attributes));
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
            (tg_device.cp_create_graphics_pipeline_state(&mut tg_graphics_pipeline_state_desc,&root_sig).unwrap(),root_sig,MaterialCBV(resources_cbv),MaterialDescTable(resource_textures))
        }
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