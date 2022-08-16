use std::mem::transmute;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use tsumugiShaderStock::{ConstantBuffer, Material, TsumugiMaterial, TsumugiShader};
use tsumuFigureStockCPU::{Attribute, Color, Joint, ObjectLoader, Texcoord, TsumugiVertexBinary, Weight};
use crate::shapell_shader_PS::ShapellShaderPS;
use crate::shapell_shader_VS::ShapellShaderVS;
use crate::test_shader_PS::TestShaderPS;
use crate::test_shader_VS::TestShaderVS;

const MaterialID: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone)]
pub struct Shapell {
    pub material: TsumugiMaterial,
}
#[derive(Clone)]
pub struct shapellbuffer{
    number:f32
}
#[derive(Clone)]
pub struct ShapellMaterial {
    pub figure_path:&'static Path,
    pub shader_path_vs:TsumugiShader,
    pub shader_path_ps:TsumugiShader,
    pub shader_path_gs: Option<TsumugiShader>,
    pub shader_path_hs: Option<TsumugiShader>,
    pub shader_path_ds: Option<TsumugiShader>,
    pub shapellBuffer:shapellbuffer,
    pub shapellTexture:Vec<&'static Path>,
    ///マテリアルの固有の番号。これが違うと異なるマテリアルと認識される
    pub material_element_id:usize,
    ///マテリアルの名前。
    pub material_name: &'static str,
}

impl Into<TsumugiMaterial> for ShapellMaterial{
    fn into(self) -> TsumugiMaterial {
        let bufarr = unsafe{
            std::mem::transmute::<_, [u8;4]>(self.shapellBuffer) };
        TsumugiMaterial{
            figure_path: self.figure_path,
            shader_path_vs: self.shader_path_ps,
            shader_path_ps: self.shader_path_ps,
            shader_path_gs: self.shader_path_gs,
            shader_path_hs: self.shader_path_hs,
            shader_path_ds: self.shader_path_ds,
            material: Material{
                texture: self.shapellTexture,
                buffer: vec![Vec::from(bufarr)],
                buffersize: 0,
                attributes: vec![]
            },
            material_element_id: self.material_element_id,
            material_name: self.material_name
        }
    }
}

///あとでbindgenみたいに自動生成できるように組む
impl ObjectLoader for Shapell {
    fn load() -> Option<TsumugiVertexBinary> {
        let (document, buffers, images) = gltf::import(Path::new("Asset/shapell_Mtoon.vrm")).unwrap_or_else(|x| { panic!("{}", x) });
        let mut binarylist = Vec::new();
        let mut indexlist = Vec::new();
        let mut attrlist: Vec<(Vec<Attribute>, u32)> = Vec::new();
        for mesh in document.meshes() {
            for prim in mesh.primitives() {
                let reader = prim.reader(|x| Some(&buffers[x.index()]));
                {
                    let mut binary: Vec<u8> = Vec::new();
                    let mut vertexes_pos = reader.read_positions()?;
                    let mut vertexes_normal = reader.read_normals()?;
                    let mut vertexes_texcoord0 = reader.read_tex_coords(0)?.into_f32();
                    for i in 0..vertexes_pos.len() {
                        let pos = vertexes_pos.next()?;
                        let bin = unsafe { std::mem::transmute::<_, [u8; 12]>(pos) };
                        //頂点情報
                        binary.append(&mut bin.to_vec());
                        //法線情報
                        binary.append(&mut (unsafe { std::mem::transmute::<_, [u8; 12]>(vertexes_normal.next()?) }).to_vec());
                        //UV座標情報
                        binary.append(&mut (unsafe { std::mem::transmute::<_, [u8; 8]>(vertexes_texcoord0.next()?) }).to_vec());
                    }
                    //ここできっちりそろえておく
                    binary.shrink_to_fit();
                    binarylist.push(binary);
                }
                {
                    let mut index_binary = reader.read_indices()?.into_u32().collect::<Vec<u32>>();
                    index_binary.shrink_to_fit();
                    indexlist.push(index_binary);
                }
                //todo:ここ削る（頂点サイズは上ですでに分かっているので）
                {
                    let attribute = vec![
                        tsumuFigureStockCPU::Attribute::Position,
                        tsumuFigureStockCPU::Attribute::Normal,
                        tsumuFigureStockCPU::Attribute::Texcoord(tsumuFigureStockCPU::Texcoord::f32)];
                    let mut vertexbytes: u32 = 0;
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
            object_path: Path::new("Asset/shapell_Mtoon.vrm"),
            shader_input_attribute: attrlist,
            vertex: binarylist,
            index: indexlist,
        });
    }
}

impl Shapell {
    ///マテリアルを新しく作る時は過去作ったものとかぶらないようにしよう
    fn new() -> Self {
        //todo:データ構造が未定。既存のマテリアルから生成するという実態に即していない可能性がある。共通のマテリアルを使いたい場合と、別々のマテリアルにしたい場合とで分ける...Defaultなくしてnew()とspawn()だけにするかもしれん
        Shapell {
            material: TsumugiMaterial {
                figure_path: Path::new("Asset/shapell_Mtoon.vrm"),
                shader_path_vs: ShapellShaderVS::load(),
                shader_path_ps: ShapellShaderPS::load(),
                shader_path_gs: None,
                shader_path_hs: None,
                shader_path_ds: None,
                material: Material {
                    texture: vec![],
                    buffer: Vec::new(),
                    buffersize: 0,
                    attributes: vec![
                        tsumuFigureStockCPU::Attribute::Position,
                        tsumuFigureStockCPU::Attribute::Normal,
                        tsumuFigureStockCPU::Attribute::Texcoord(tsumuFigureStockCPU::Texcoord::f32)
                    ]
                },
                material_element_id: MaterialID.fetch_add(1, Ordering::SeqCst),
                material_name: "ShapellMaterial"
            },
        }
    }
}

impl Default for Shapell {
    fn default() -> Self {
        Shapell {
            material: TsumugiMaterial {
                figure_path: Path::new("Asset/shapell_Mtoon.vrm"),
                shader_path_vs: ShapellShaderVS::load(),
                shader_path_ps: ShapellShaderPS::load(),
                shader_path_gs: None,
                shader_path_hs: None,
                shader_path_ds: None,
                material: Material {
                    ///テクスチャを事前に生成しておこう。
                    texture: vec![
                        Path::new("Asset/shapell_Mtoon_img0.png"),
                        Path::new("Asset/shapell_Mtoon_img1.png"),
                        Path::new("Asset/shapell_Mtoon_img2.png"),
                        Path::new("Asset/shapell_Mtoon_img3.png"),
                        Path::new("Asset/shapell_Mtoon_img4.png"),
                        Path::new("Asset/shapell_Mtoon_img5.png")],
                    //1.5の浮動小数点表現（リトルエンディアン）
                    buffer: vec![vec![0x00,0x00,0xc0,0x3f]],
                    buffersize: 4,
                    attributes: vec![
                        tsumuFigureStockCPU::Attribute::Position,
                        tsumuFigureStockCPU::Attribute::Normal,
                        tsumuFigureStockCPU::Attribute::Texcoord(tsumuFigureStockCPU::Texcoord::f32)
                    ]
                },
                material_element_id: 0,
                material_name: "ShapellMaterial",
            },
        }
    }
}
