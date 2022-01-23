use std::fs::File;
use std::io::Read;
use std::path::Path;
use tsumugiShaderStock::TsumugiShader;

static _shader: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "Asset/PixelShader.cso"));
const shader: &[u8] = include_bytes!("../Asset/shapell_PS.cso");
#[derive(Clone)]
pub struct ShapellShaderPS {
    tsumugi_shader:TsumugiShader
}

impl ShapellShaderPS {
    pub fn load()->TsumugiShader{
        TsumugiShader{
            shader_path: Path::new("Asset/shapell_PS.cso"),
            shader_size: shader.len(),
            shader_pointer: shader
        }
    }
}