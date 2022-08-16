use std::ptr::{null, null_mut};
use winapi::shared::minwindef::UINT;
use winapi::um::d3d12::{D3D12_DESCRIPTOR_RANGE, D3D12_ROOT_CONSTANTS, D3D12_ROOT_DESCRIPTOR, D3D12_ROOT_DESCRIPTOR_TABLE, D3D12_ROOT_PARAMETER, D3D12_ROOT_PARAMETER_TYPE_32BIT_CONSTANTS, D3D12_ROOT_PARAMETER_TYPE_CBV, D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE, D3D12_ROOT_PARAMETER_TYPE_SRV, D3D12_ROOT_PARAMETER_TYPE_UAV, D3D12_ROOT_PARAMETER_u, D3D12_ROOT_SIGNATURE_DESC, D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT, D3D12_ROOT_SIGNATURE_FLAG_NONE, D3D12_ROOT_SIGNATURE_FLAGS, D3D12_SHADER_VISIBILITY, D3D12_SHADER_VISIBILITY_ALL, D3D12_STATIC_SAMPLER_DESC, D3D12SerializeRootSignature, D3D_ROOT_SIGNATURE_VERSION};
use winapi::um::d3dcommon::ID3D10Blob;
use winapi::um::winnt::HRESULT;
use tsugumi_windows_library::HRESULTinto;
use crate::tg_directx::CpID3DBlob;

pub struct TgD3d12RootSignatureDesc(pub(crate) D3D12_ROOT_SIGNATURE_DESC);

pub struct TgD3d12RootParameter(pub(crate) D3D12_ROOT_PARAMETER);

pub struct TgD3d12RootParameters(pub(crate) Vec<D3D12_ROOT_PARAMETER>);
//todo:いままでenumを使っていたところを少しづつ変えていこう
union D3d12RootParameterUnion {
    d3d12_root_descriptor_table: D3D12_ROOT_DESCRIPTOR_TABLE,
    d3d12_root_descriptor: D3D12_ROOT_DESCRIPTOR,
    d3d12_root_constants: D3D12_ROOT_CONSTANTS,
}

impl TgD3d12RootParameter {
    ///Descriptor Tableを作るよ。
    pub(crate) fn create_descriptor_table(table: &Vec<D3D12_DESCRIPTOR_RANGE>, d3d12_shader_visibility:D3D12_SHADER_VISIBILITY) -> D3D12_ROOT_PARAMETER {
        D3D12_ROOT_PARAMETER {
            ParameterType: D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE,
            u: unsafe {
                std::mem::transmute::<_, _>(D3d12RootParameterUnion {
                    d3d12_root_descriptor_table: D3D12_ROOT_DESCRIPTOR_TABLE {
                        NumDescriptorRanges: table.len() as UINT,
                        pDescriptorRanges: table.as_ptr(),
                    }
                }
                )
            },
            ShaderVisibility: d3d12_shader_visibility,
        }
    }
    ///Descriptor CBV,SRV,UAVを作るよ。初期状態ではCBVなので各自ParameterTypeを変更して対応するように
    pub(crate) fn create_descriptor_views(buffer: D3D12_ROOT_DESCRIPTOR, d3d12_shader_visibility:D3D12_SHADER_VISIBILITY) -> D3D12_ROOT_PARAMETER {
        D3D12_ROOT_PARAMETER {
            ParameterType: D3D12_ROOT_PARAMETER_TYPE_CBV,
            u: unsafe {
                //サイズが違う（128bit、64bit）ので無理やりサイズを合わせて送る。transmute_copyは本当にやばいので見送った
                std::mem::transmute::<_, _>(D3d12RootParameterUnion { d3d12_root_descriptor: buffer })
            },
            ShaderVisibility: d3d12_shader_visibility,
        }
    }
    ///Descriptor ROOT_CONSTANTSを作るよ。
    pub(crate) fn create_descriptor_ROOT_CONSTANTS(buffer: D3D12_ROOT_CONSTANTS, d3d12_shader_visibility:D3D12_SHADER_VISIBILITY) -> D3D12_ROOT_PARAMETER {
        D3D12_ROOT_PARAMETER {
            ParameterType: D3D12_ROOT_PARAMETER_TYPE_32BIT_CONSTANTS,
            u: unsafe {
                //サイズが違う（128bit、64bit）ので無理やりサイズを合わせて送る。transmute_copyは本当にやばいので見送った
                std::mem::transmute::<_, _>(D3d12RootParameterUnion { d3d12_root_constants: buffer })
            },
            ShaderVisibility: d3d12_shader_visibility,
        }
    }
}

impl TgD3d12RootParameters {
    pub fn with_capacity(capacity: usize) -> Self {
        TgD3d12RootParameters { 0: Vec::with_capacity(capacity) }
    }
    pub fn append_descriptor_table(&mut self, table: &Vec<D3D12_DESCRIPTOR_RANGE>, d3d12_shader_visibility:D3D12_SHADER_VISIBILITY) {
        self.0.push(TgD3d12RootParameter::create_descriptor_table(table, d3d12_shader_visibility));
    }
    pub fn append_descriptor_cbv(&mut self, buffer: D3D12_ROOT_DESCRIPTOR, d3d12_shader_visibility:D3D12_SHADER_VISIBILITY) {
        let mut root_parameter = TgD3d12RootParameter::create_descriptor_views(buffer,d3d12_shader_visibility);
        root_parameter.ParameterType = D3D12_ROOT_PARAMETER_TYPE_CBV;
        self.0.push(root_parameter);
    }
    pub fn append_descriptor_srv(&mut self, buffer: D3D12_ROOT_DESCRIPTOR, d3d12_shader_visibility:D3D12_SHADER_VISIBILITY) {
        let mut root_parameter = TgD3d12RootParameter::create_descriptor_views(buffer,d3d12_shader_visibility);
        root_parameter.ParameterType = D3D12_ROOT_PARAMETER_TYPE_SRV;
        self.0.push(root_parameter);
    }
    pub fn append_descriptor_uav(&mut self, buffer: D3D12_ROOT_DESCRIPTOR, d3d12_shader_visibility:D3D12_SHADER_VISIBILITY) {
        let mut root_parameter = TgD3d12RootParameter::create_descriptor_views(buffer,d3d12_shader_visibility);
        root_parameter.ParameterType = D3D12_ROOT_PARAMETER_TYPE_UAV;
        self.0.push(root_parameter);
    }
    pub fn append_descriptor_root_constants(&mut self, buffer: D3D12_ROOT_CONSTANTS, d3d12_shader_visibility:D3D12_SHADER_VISIBILITY) {
        self.0.push(TgD3d12RootParameter::create_descriptor_ROOT_CONSTANTS(buffer, d3d12_shader_visibility));
    }
}

impl TgD3d12RootSignatureDesc {
    pub fn cp_d3d12serialize_root_signature(&self, version: D3D_ROOT_SIGNATURE_VERSION) -> Result<CpID3DBlob, (CpID3DBlob, HRESULT)> {
        let mut okBlob: *mut ID3D10Blob = null_mut();
        let mut errBlob: *mut ID3D10Blob = null_mut();
        unsafe {
            match D3D12SerializeRootSignature(&self.0, version, &mut okBlob, &mut errBlob).result() {
                Ok(_) => return Ok(CpID3DBlob(okBlob.as_mut().unwrap())),
                Err(v) => return Err((CpID3DBlob(errBlob.as_mut().unwrap()), v))
            }
        }
    }
    pub fn root_parameter(mut self, tg_d3d12root_parameters:&TgD3d12RootParameters) ->Self{
        self.0.NumParameters = tg_d3d12root_parameters.0.len() as UINT;
        self.0.pParameters = tg_d3d12root_parameters.0.as_ptr();
        self
    }
    pub fn static_sampler(mut self,static_sampler:&Vec<D3D12_STATIC_SAMPLER_DESC>)->Self{
        self.0.NumStaticSamplers = static_sampler.len() as UINT;
        self.0.pStaticSamplers = static_sampler.as_ptr();
        self
    }
    pub fn flag(mut self,flag:D3D12_ROOT_SIGNATURE_FLAGS)->Self{
        self.0.Flags |= flag;
        self
    }
}

impl Default for TgD3d12RootSignatureDesc {
    fn default() -> Self {
        let d3d12_root_signature_desc = D3D12_ROOT_SIGNATURE_DESC {
            NumParameters: 0,
            pParameters: null(),
            NumStaticSamplers: 0,
            pStaticSamplers: null(),
            Flags: D3D12_ROOT_SIGNATURE_FLAG_NONE,
        };
        TgD3d12RootSignatureDesc(d3d12_root_signature_desc)
    }
}