use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use tsumugiShaderStock::{Material, TsumugiMaterial};
use tsumuFigureStockCPU::{Attribute, Color, Joint, ObjectLoader, Texcoord, TsumugiVertexBinary, Weight};
use crate::test_shader_PS::TestShaderPS;
use crate::test_shader_VS::TestShaderVS;

const MaterialID:AtomicU64 = AtomicU64::new(1);
#[derive(Clone)]
pub struct SampleBox {
    pub material:TsumugiMaterial
}
///あとでbindgenみたいに自動生成できるように組む
impl ObjectLoader for SampleBox {
    fn load() -> Option<TsumugiVertexBinary> {
        let (document, buffers, images) = gltf::import(Path::new("Asset/Box.glb")).unwrap_or_else(|x| { panic!("{}", x) });
        let mut binarylist = Vec::new();
        let mut indexlist = Vec::new();
        let mut attrlist = Vec::new();
        for mesh in document.meshes() {
            for prim in mesh.primitives() {
                let reader = prim.reader(|x| Some(&buffers[x.index()]));
                {
                    let mut binary: Vec<u8> = Vec::new();
                    let mut vertexes_pos = reader.read_positions()?;
                    let mut vertexes_normal = reader.read_normals()?;
                    for i in 0..vertexes_pos.len() {
                        let pos = vertexes_pos.next()?;
                        let bin = unsafe{std::mem::transmute::<_,[u8;12]>(pos)};
                        binary.append(&mut bin.to_vec());
                        binary.append(&mut (unsafe{std::mem::transmute::<_,[u8;12]>(vertexes_normal.next()?)}).to_vec());
                    }
                    binary.shrink_to_fit();
                    binarylist.push(binary);
                }
                {
                    let mut index_binary = reader.read_indices()?.into_u32().collect::<Vec<u32>>();
                    index_binary.shrink_to_fit();
                    indexlist.push(index_binary);
                }
                {
                    let attribute = vec![
                        tsumuFigureStockCPU::Attribute::Position,
                        tsumuFigureStockCPU::Attribute::Normal
                    ];
                    let mut vertexbytes = 0;
                    for attr in &attribute {
                        match attr {
                            Attribute::Position => {
                                vertexbytes += 12;
                            }
                            Attribute::Normal => {
                                vertexbytes += 12;
                            }
                            Attribute::Tangent => {
                                vertexbytes += 16;
                            }
                            Attribute::Color(Color::RGBA_f32) => {
                                vertexbytes += 16;
                            }
                            Attribute::Texcoord(Texcoord::f32) => {
                                vertexbytes += 8;
                            }
                            Attribute::Joint(Joint::u16) => {
                                vertexbytes += 4;
                            }
                            Attribute::Weight(Weight::f32) => {
                                vertexbytes += 16;
                            }
                            _ => { todo!("まだここはできてないよ") }
                        }
                    }
                    attrlist.push((attribute, vertexbytes))
                }
            }
        }
        return Some(TsumugiVertexBinary {
            object_path: Path::new("Asset/Box.glb"),
            shader_input_attribute: attrlist,
            vertex: binarylist,
            index: indexlist,
        });
    }
}

impl SampleBox {
    fn new()->Self{
        SampleBox {
            material: TsumugiMaterial {
                figure_path: Path::new("Asset/Box.glb"),
                shader_path_vs: TestShaderVS::load(),
                shader_path_ps: TestShaderPS::load(),
                shader_path_gs: None,
                shader_path_hs: None,
                shader_path_ds: None,
                material: Material {
                    texture: vec![],
                    buffer: Vec::new(),
                    buffersize: 0,
                    attributes: vec![
                        tsumuFigureStockCPU::Attribute::Position,
                        tsumuFigureStockCPU::Attribute::Normal
                    ]
                },
                material_element_id: MaterialID.fetch_add(1, Ordering::SeqCst),
                material_name: "SampleBoxMaterial",
            },
        }
    }
}

impl Default for SampleBox {
    fn default() -> Self {
        SampleBox {
            material: TsumugiMaterial {
                figure_path: Path::new("Asset/Box.glb"),
                shader_path_vs: TestShaderVS::load(),
                shader_path_ps: TestShaderPS::load(),
                shader_path_gs: None,
                shader_path_hs: None,
                shader_path_ds: None,
                material: Material {
                    texture: vec![],
                    buffer: Vec::new(),
                    buffersize: 0,
                    attributes: vec![
                        tsumuFigureStockCPU::Attribute::Position,
                        tsumuFigureStockCPU::Attribute::Normal
                    ]
                },
                material_element_id: 0,
                material_name: "SampleBoxMaterial",
            },
        }
    }
}
