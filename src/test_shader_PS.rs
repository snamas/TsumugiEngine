use std::fs::File;
use std::io::Read;
use std::path::Path;
use tsumugiShaderStock::TsumugiShader;

static _shader: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "Asset/PixelShader.cso"));
const shader: &[u8] = include_bytes!("../Asset/PixelShader.cso");
#[derive(Clone)]
pub struct TestShaderPS {
    tsumugi_shader:TsumugiShader
}

impl TestShaderPS {
    pub fn load()->TsumugiShader{
        TsumugiShader{
            shader_path: Path::new("Asset/PixelShader.cso"),
            shader_size: shader.len(),
            shader_pointer: shader
        }
    }
}