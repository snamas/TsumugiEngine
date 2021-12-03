use std::ptr::{null, null_mut};
use winapi::shared::minwindef::UINT;
use winapi::um::d3d12::{D3D12_DESCRIPTOR_RANGE, D3D12_ROOT_DESCRIPTOR, D3D12_ROOT_DESCRIPTOR_TABLE, D3D12_ROOT_PARAMETER, D3D12_ROOT_PARAMETER_TYPE_CBV, D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE, D3D12_ROOT_PARAMETER_u, D3D12_ROOT_SIGNATURE_DESC, D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT, D3D12_SHADER_VISIBILITY_ALL, D3D12SerializeRootSignature, D3D_ROOT_SIGNATURE_VERSION};
use winapi::um::d3dcommon::ID3D10Blob;
use winapi::um::winnt::HRESULT;
use tsugumi_windows_library::HRESULTinto;
use crate::tg_directx::CpID3DBlob;

pub struct TgD3d12RootSignatureDesc(pub(crate) D3D12_ROOT_SIGNATURE_DESC);

pub struct TgD3d12RootParameter(pub(crate) D3D12_ROOT_PARAMETER);

impl TgD3d12RootParameter {
    pub fn create_from_descriptor_table(table: Vec<D3D12_DESCRIPTOR_RANGE>) -> Self {
        TgD3d12RootParameter(D3D12_ROOT_PARAMETER {
            ParameterType: D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE,
            u: unsafe {
                std::mem::transmute::<_, _>(
                    D3D12_ROOT_DESCRIPTOR_TABLE {
                        NumDescriptorRanges: table.len() as UINT,
                        pDescriptorRanges: table.as_ptr(),
                    })
            },
            ShaderVisibility: D3D12_SHADER_VISIBILITY_ALL,
        })
    }
    pub fn create_from_descriptor_CBV(buffer:D3D12_ROOT_DESCRIPTOR)->Self{
        TgD3d12RootParameter(D3D12_ROOT_PARAMETER {
            ParameterType: D3D12_ROOT_PARAMETER_TYPE_CBV,
            u: unsafe {
                //サイズが違う（128bit、64bit）ので無理やりサイズを合わせて送る。transmute_copyは本当にやばいので見送った
                std::mem::transmute::<_, _>([buffer,buffer])
            },
            ShaderVisibility: D3D12_SHADER_VISIBILITY_ALL,
        })

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
}

impl Default for TgD3d12RootSignatureDesc {
    fn default() -> Self {
        let d3d12_root_signature_desc = D3D12_ROOT_SIGNATURE_DESC {
            NumParameters: 0,
            pParameters: null(),
            NumStaticSamplers: 0,
            pStaticSamplers: null(),
            Flags: D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
        };
        TgD3d12RootSignatureDesc(d3d12_root_signature_desc)
    }
}