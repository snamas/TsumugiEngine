use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tsumuFigureStockCPU::Attribute;
use tsumugi::controller::{TsumugiPortal, TsumugiPortalPlaneLocal, TsumugiControllerTrait, TsumugiObject};
use tsumugi::controller::TsumugiControllerItemLifeTime::{Eternal, Once};
use tsumugi::controller::TsumugiControllerItemState::Fulfilled;
use tsumugi::distributor::TsumugiParcelDistributor;
use tsumugi::parcelreceptor_novalue::TsumugiParcelReceptorNoVal;

static TSUMUGI_STOCK_MATERIAL_NAME: &str = "TsumugiStockMaterials";

#[derive(Clone)]
pub struct TsumugiMaterial {
    pub figure_path:&'static Path,
    pub shader_path_vs:TsumugiShader,
    pub shader_path_ps:TsumugiShader,
    pub shader_path_gs: Option<TsumugiShader>,
    pub shader_path_hs: Option<TsumugiShader>,
    pub shader_path_ds: Option<TsumugiShader>,
    ///マテリアルの名前が重複していた場合は古いマテリアルが消される。
    pub material:Material,
    ///マテリアルの固有の番号。これが違うと異なるマテリアルと認識される
    pub material_element_id:usize,
    ///マテリアルの名前。現状なにも起こらない
    pub material_name: &'static str,
}
#[derive(Clone)]
pub enum ConstantBuffer{
    F32(f32),
    F32_4(f32,f32,f32,f32)
}
#[derive(Clone)]
pub struct CBuffer(Vec<ConstantBuffer>);
#[derive(Clone)]
pub struct Material{
    ///テクスチャ配列
    pub texture:Vec<&'static Path>,
    ///バッファ配列。複数のCBufferバイナリデータで構成される。
    pub buffer: Vec<Vec<u8>>,
    ///事前に確定しておいたバッファ配列から計算したバッファのバイトサイズ。リソース作成の時に使う。
    pub buffersize:usize,
    ///一つの頂点データに何が含まれているか（頂点の場所、法線、ジョイント...）
    pub attributes:Vec<Attribute>
}
#[derive(Clone,Copy)]
pub struct TsumugiShader {
    pub shader_path:&'static Path,
    pub shader_size:usize,
    pub shader_pointer:&'static [u8]
}


///PathはオブジェクトのパスをカギとしたHashMap
#[derive(Clone)]
pub struct TsumugiShaderStockController(pub Arc<Mutex<HashMap<&'static str, HashMap<&'static Path,TsumugiMaterial>>>>);

impl TsumugiMaterial {
    pub fn store_material(&self,tc:&TsumugiPortal){
        let material_dist = TsumugiParcelDistributor::new(self.clone()).lifetime(Once).displayname("material_list");
        tc.find(TSUMUGI_STOCK_MATERIAL_NAME).unwrap().distributor_channel_sender.send(material_dist.into());
    }
}

impl Default for TsumugiShaderStockController {
    fn default() -> Self {
        TsumugiShaderStockController { 0: Arc::new(Mutex::new(Default::default())) }
    }
}
impl TsumugiObject for TsumugiShaderStockController {
    fn on_create(&self, tc: &TsumugiPortalPlaneLocal) {
        let mut shaderList = self.0.clone();
        let antenna = TsumugiParcelReceptorNoVal::<TsumugiMaterial>::new().subscribe_with_portal(Arc::new(move |parcel, tc|{
            let parcel = parcel.parcel.clone().unwrap();
            //ここマテリアルの転換が必要か
            let name = parcel.material_name;
            let path  = parcel.figure_path;
            let material = *parcel;
            //todo:ここマテリアルをこのPlaneに置く必要がまるでなくなってしまったよ
            shaderList.lock().unwrap().entry(name).or_insert(HashMap::new()).insert(path,material.clone());
            //マテリアルをそのままクローンして送ると、アンテナに引っかかってまたマテリアルがクローンされるループに引っかかってしまった。
            tc.tp.local_channel_sender.distributor_channel_sender.send(TsumugiParcelDistributor::new(material.clone()).lifetime(Once).parcelname("re_material").displayname("material").into());
            Fulfilled
        })).to_antenna().displayname("recept_material").lifetime(Eternal);
        tc.tp.local_channel_sender.recept_channel_sender.send(antenna.into());
        tc.tp.local_channel_sender.distributor_channel_sender.send(TsumugiParcelDistributor::new(self.clone()).lifetime(Eternal).displayname("TsumugiShaderController").into());
    }
}
pub fn spawn_shader_stock_handler(tc:&Box<TsumugiPortal>) -> Box<TsumugiPortal>{
    let mut newtc = tc.spawn(TSUMUGI_STOCK_MATERIAL_NAME.to_string());
    newtc.set_objects(vec![
        Box::new(TsumugiShaderStockController::default()),
    ]);
    return newtc;
}