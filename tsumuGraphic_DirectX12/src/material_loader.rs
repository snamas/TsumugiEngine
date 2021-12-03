use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{Error, Read};
use std::path::Path;
use std::sync::{Arc, Mutex};
use winapi::shared::minwindef::UINT;
use winapi::um::d3d12::{D3D12_DESCRIPTOR_RANGE, D3D12_DESCRIPTOR_RANGE_TYPE_SRV, D3D12_ROOT_PARAMETER, D3D12_STATIC_SAMPLER_DESC, D3D_ROOT_SIGNATURE_VERSION_1_0};
use tsumugiShaderStock::TsumugiMaterial;
use crate::tg_graphics_pipeline::TgD3d12GraphicsPipeline;
use crate::tg_root_signature::TgD3d12RootSignatureDesc;
use crate::tg_sampler::TgStaticSamplerDesc;
use crate::TgID3D12Device;

pub(crate) trait MaterialLoadDirectx12 {
    fn load(&self, tg_device: Arc<TgID3D12Device>);
}

impl MaterialLoadDirectx12 for TsumugiMaterial {
    fn load(&self, tg_device: Arc<TgID3D12Device>){
        let constant_buffer_len = self.material.buffer.len();
        let discriptor_range:Vec<D3D12_DESCRIPTOR_RANGE> = vec![
            D3D12_DESCRIPTOR_RANGE{
                RangeType: D3D12_DESCRIPTOR_RANGE_TYPE_SRV,
                NumDescriptors: self.material.texture.len() as UINT,
                BaseShaderRegister: 0,
                RegisterSpace: 0,
                OffsetInDescriptorsFromTableStart: 0
        }];
        let mut sampler:Vec<D3D12_STATIC_SAMPLER_DESC> = Vec::with_capacity(self.material.texture.len());
        for index in 0..self.material.texture.len(){
            sampler.push(TgStaticSamplerDesc::default().shader_register(index as u32).0);
        }
        //todo:ここテクスチャが一枚も入って無くても１確保される
        let root_parameter:Vec<D3D12_ROOT_PARAMETER> = Vec::with_capacity(constant_buffer_len+1);
        //todo:シグネチャーどんどん作るよ

        let cp_d3d12_root_signature_desc: TgD3d12RootSignatureDesc = Default::default();
        //todo:ここマテリアルの属性をいろいろ入れたいね（現在シェーダー入れるだけ）
        let mut cpgraphicsPipelineStateDesc = TgD3d12GraphicsPipeline::default()
            .VertexShader(&self.shader_path_vs).PixelShader(&self.shader_path_ps);
        if let Some(gs) = &self.shader_path_gs{
            cpgraphicsPipelineStateDesc = cpgraphicsPipelineStateDesc.Geometryhader(gs);
        }
        if let Some(hs) = &self.shader_path_hs{
            cpgraphicsPipelineStateDesc = cpgraphicsPipelineStateDesc.Geometryhader(hs);
        }
        if let Some(ds) = &self.shader_path_ds {
            cpgraphicsPipelineStateDesc = cpgraphicsPipelineStateDesc.Geometryhader(ds);
        }

    }
}