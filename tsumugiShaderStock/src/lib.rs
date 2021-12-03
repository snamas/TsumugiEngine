use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tsumugi::controller::{TsumugiController, TsumugiController_threadlocal, TsumugiControllerTrait, TsumugiObject};
use tsumugi::controller::TsumugiControllerItemLifeTime::{Eternal, Once};
use tsumugi::controller::TsumugiControllerItemState::Fulfilled;
use tsumugi::distributor::TsumugiParcelDistributor;
use tsumugi::parcelreceptor_novalue::TsumugiParcelReceptorNoVal;

static TSUMUGI_STOCK_SHADER_NAME: &str = "TsumugiStockShader";
#[derive(Clone)]
pub struct TsumugiMaterial {
    pub material_name:&'static str,
    pub shader_path_vs:TsumugiShader,
    pub shader_path_ps:TsumugiShader,
    pub shader_path_gs: Option<TsumugiShader>,
    pub shader_path_hs: Option<TsumugiShader>,
    pub shader_path_ds: Option<TsumugiShader>,
    pub material:Material
}
#[derive(Clone)]
pub enum ConstantBuffer{
    F32(f32),
    F32_4([f32;4])
}
#[derive(Clone)]
pub struct CBuffer(Vec<ConstantBuffer>);
#[derive(Clone)]
pub struct Material{
    pub texture:Vec<&'static Path>,
    pub buffer: Vec<CBuffer>,
    pub material_element_id:u64
}
#[derive(Clone,Copy)]
pub struct TsumugiShader {
    pub shader_path:&'static Path,
    pub shader_size:usize,
    pub shader_pointer:&'static [u8]
}


///PathはオブジェクトのパスをカギとしたHashMap
#[derive(Clone)]
pub struct TsumugiShaderStockController(pub Arc<Mutex<HashMap<&'static str, TsumugiMaterial>>>);

impl TsumugiMaterial {
    pub fn store_material(&self,tc:&TsumugiController){
        let material_dist = TsumugiParcelDistributor::new(self.clone()).lifetime(Once).displayname("material");
        tc.find(TSUMUGI_STOCK_SHADER_NAME).unwrap().pickup_channel_sender.send(material_dist.into());
    }
}

impl Default for TsumugiShaderStockController {
    fn default() -> Self {
        TsumugiShaderStockController { 0: Arc::new(Mutex::new(Default::default())) }
    }
}
impl TsumugiObject for TsumugiShaderStockController {
    fn on_create(&self, tc: &TsumugiController_threadlocal) {
        let mut shaderList = self.0.clone();
        let antenna = TsumugiParcelReceptorNoVal::<TsumugiMaterial>::new().subscribe(Arc::new(move |parcel|{
            let parcel = parcel.parcel.clone().unwrap();
            let name = parcel.material_name;
            let material = *parcel;
            shaderList.lock().unwrap().insert(name,material);
            Fulfilled
        })).to_antenna().displayname("recept_material").lifetime(Eternal);
        tc.tc.local_channel_sender.recept_channel_sender.send(antenna.into());
        tc.tc.local_channel_sender.pickup_channel_sender.send(TsumugiParcelDistributor::new(self.clone()).lifetime(Eternal).displayname("TsumugiShaderController").into());

    }
}
pub fn spown_shader_stock_handler(tc:&Box<TsumugiController>) -> Box<TsumugiController>{
    let mut newtc = tc.spown(TSUMUGI_STOCK_SHADER_NAME.to_string());
    newtc.set_objects(vec![
        Box::new(TsumugiShaderStockController::default()),
    ]);
    return newtc;
}