use std::fs::File;
use std::io::Read;
use std::path::Path;
use tsumugiShaderStock::TsumugiShader;

static _shader: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "Asset/test_compiled_VS.cso"));
const shader: &[u8] = include_bytes!("../Asset/test_compiled_VS.cso");

#[derive(Clone)]
pub struct TestShaderVS {
    tsumugi_shader:TsumugiShader
}

impl TestShaderVS {
    pub fn load()->TsumugiShader{
        TsumugiShader{
            shader_path: Path::new("Asset/test_compiled_VS.cso"),
            shader_size: shader.len(),
            shader_pointer: shader
        }
    }
}