use std::collections::HashMap;
use std::fmt::Binary;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::Thread;
use gltf::{buffer, Document, image};
use tsumugi::controller::{TsumugiController, TsumugiController_thread, TsumugiControllerItemLifeTime, TsumugiControllerItemState, TsumugiControllerTrait, TsumugiObject};
use tsumugi::distributor::TsumugiParcelDistributor;
use tsumugi::parcel_receptor::TsumugiParcelReceptor;

#[derive(Copy, Clone)]
pub enum Attribute {
    Position,
    Normal,
    Tangent,
    Color(Color),
    Texcoord(Texcoord),
    Joint(Joint),
    Weight(Weight)
}
#[derive(Copy, Clone)]
pub enum Color {
    RGB_u8,
    RGB_u16,
    RGB_f32,
    RGBA_u8,
    RGBA_u16,
    RGBA_f32,
}
#[derive(Copy, Clone)]
pub enum Texcoord {
    u8,
    u16,
    f32
}
#[derive(Copy, Clone)]
pub enum Joint{
    u8,
    u16
}
#[derive(Copy, Clone)]
pub enum Weight{
    u8,
    u16,
    f32
}
#[derive(Clone)]
pub struct TsumugiVertexBinary {
    pub object_path:&'static Path,
    pub shader_input_attribute:Vec<(Vec<Attribute>,u32)>,
    pub vertex:Vec<Vec<u8>>,
    pub index:Vec<Vec<u32>>
}
#[derive(Clone)]
pub struct TsumugiMaterial {
    pub shader_path:&'static Path,
    pub material:Material
}
#[derive(Clone)]
pub struct Material{
    pub texture:Vec<&'static Path>,
    pub f32:Vec<f32>,
    pub f32_4:Vec<[f32;4]>,
    pub material_element_id:u32
}


pub trait ObjectLoader {
    fn load()->Option<TsumugiVertexBinary>;
}

struct tsumugiStock();

static TSUMUGI_STOCK_CPUNAME: &str = "TsumugiStockCPU";
///PathはオブジェクトのパスをカギとしたHashMap
#[derive(Clone)]
pub struct TsumugiStockController(pub Arc<Mutex<HashMap<&'static Path, Arc<TsumugiVertexBinary>>>>);

#[derive(Clone, Copy)]
pub struct TsumugiStock(pub &'static Path, pub fn() -> Option<TsumugiVertexBinary>);

impl TsumugiStockController {
    ///オブジェクトを保存するよ
    fn store(&self, stock_element: TsumugiStock,tc:&TsumugiController_thread)->Option<&'static Path> {
        let thread_arc = self.clone();
        let stock = stock_element.clone();
        let thread_tc = tc.tc.clone();
        //todo:ここ雑にロードのマルチスレッド化を行っている
        thread::spawn(move ||{
            //ここでオブジェクトのロードがあるよ
            let figure_data = stock.1().unwrap();
            thread_arc.0.lock().unwrap().insert(stock.0, Arc::from(figure_data));
            Self::announce(stock_element.0,&thread_tc);
        });
        return Some(stock_element.0);
    }
    ///オブジェクトを保存して、バイナリを返すよ。
    fn load_store_sync(&self, stock_element: TsumugiStock) -> Option<Arc<TsumugiVertexBinary>> {
        let material = Arc::new(stock_element.1()?);
        self.0.lock().unwrap().insert(stock_element.0, material.clone());
        Some(material)
    }
    ///オブジェクト引っ張ってくるよ、あったらそのまま返して無かったら生成して返すよ。
    fn load_sync(&self, path: TsumugiStock) -> Option<Arc<TsumugiVertexBinary>> {
        if let Some(value) = self.0.lock().unwrap().get(path.0) {
            Some(value.clone())
        } else {
            self.load_store_sync(path)
        }
    }
    fn announce(figure_path:&'static Path,tc:&TsumugiController){
        //オブジェクトが生成されたら、生成されたことを「周知」させる
        tc.local_channel_sender.pickup_channel_sender.send(TsumugiParcelDistributor::new(figure_path).lifetime(TsumugiControllerItemLifeTime::Once).displayname("announce").into());
    }
}

impl Default for TsumugiStockController {
    fn default() -> Self {
        TsumugiStockController { 0: Arc::new(Mutex::new(Default::default())) }
    }
}
fn none()->Option<TsumugiVertexBinary>{
    None
}
impl TsumugiObject for TsumugiStockController {
    fn on_create(&self, tc: &tsumugi::controller::TsumugiController_thread) {
        let mut object_hashmap = self.clone();
        let recept_object = TsumugiParcelReceptor::new(TsumugiStock { 0: &Path::new(""), 1: none })
            .subscribe_tc(Arc::new(move |object,tc| {
                //todo:ここキーをどうするか未定
                object_hashmap.store(*object.parcel,tc);
                TsumugiControllerItemState::Fulfilled
            })).to_antenna().displayname("recept_object");
        let dist_stock = TsumugiParcelDistributor::new(self.clone()).lifetime(TsumugiControllerItemLifeTime::Eternal).displayname("TsumugiStockController");
        tc.tc.local_channel_sender.pickup_channel_sender.send(dist_stock.into());
        tc.tc.local_channel_sender.recept_channel_sender.send(recept_object.into());
    }
}

impl TsumugiStock {
    pub fn store_object(&self, tc: &TsumugiController) {
        let tsumugi_path = self.clone();
        let path_dist = TsumugiParcelDistributor::new(tsumugi_path);
        tc.find(TSUMUGI_STOCK_CPUNAME).unwrap().pickup_channel_sender.send(path_dist.into());
    }
}


pub fn spown_object_stock_handler(tc: &Box<TsumugiController>) -> Box<TsumugiController> {
    let mut newtc = tc.spown(TSUMUGI_STOCK_CPUNAME.to_string());
    newtc.set_objects(vec![
        Box::new(TsumugiStockController::default()),
    ]);
    return newtc;
}
