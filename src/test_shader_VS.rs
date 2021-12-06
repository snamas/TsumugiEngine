use std::fs::File;
use std::io::Read;
use std::path::Path;
use tsumugiShaderStock::TsumugiShader;

static _shader: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "Asset/VertexShader.cso"));
const shader: &[u8] = include_bytes!("../Asset/VertexShader.cso");

#[derive(Clone)]
pub struct TestShaderVS {
    tsumugi_shader:TsumugiShader
}

impl TestShaderVS {
    pub fn load()->TsumugiShader{
        TsumugiShader{
            shader_path: Path::new("Asset/VertexShader.cso"),
            shader_size: shader.len(),
            shader_pointer: shader
        }
    }
}