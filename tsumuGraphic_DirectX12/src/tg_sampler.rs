use winapi::shared::minwindef::UINT;
use winapi::um::d3d12::{D3D12_COMPARISON_FUNC_NEVER, D3D12_DEFAULT_MIP_LOD_BIAS, D3D12_FILTER_MIN_MAG_MIP_LINEAR, D3D12_SHADER_VISIBILITY_PIXEL, D3D12_STATIC_BORDER_COLOR_TRANSPARENT_BLACK, D3D12_STATIC_SAMPLER_DESC, D3D12_TEXTURE_ADDRESS_MODE_CLAMP};

pub struct TgStaticSamplerDesc(pub(crate) D3D12_STATIC_SAMPLER_DESC);

impl TgStaticSamplerDesc {
    pub fn shader_register(mut self,number:u32)->Self{
        self.0.ShaderRegister = number;
        self
    }
}

impl Default for TgStaticSamplerDesc {
    fn default() -> Self {
        TgStaticSamplerDesc(D3D12_STATIC_SAMPLER_DESC{
            Filter: D3D12_FILTER_MIN_MAG_MIP_LINEAR,
            AddressU: D3D12_TEXTURE_ADDRESS_MODE_CLAMP,
            AddressV: D3D12_TEXTURE_ADDRESS_MODE_CLAMP,
            AddressW: D3D12_TEXTURE_ADDRESS_MODE_CLAMP,
            MipLODBias: D3D12_DEFAULT_MIP_LOD_BIAS,
            MaxAnisotropy: 1,
            ComparisonFunc: D3D12_COMPARISON_FUNC_NEVER,
            BorderColor: D3D12_STATIC_BORDER_COLOR_TRANSPARENT_BLACK,
            MinLOD: f32::MIN,
            MaxLOD: f32::MAX,
            ShaderRegister: 0,
            RegisterSpace: 0,
            ShaderVisibility: D3D12_SHADER_VISIBILITY_PIXEL
        })
    }
}