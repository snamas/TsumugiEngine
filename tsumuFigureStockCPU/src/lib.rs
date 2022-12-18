use std::collections::HashMap;
use std::fmt::Binary;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::Thread;
use gltf::{buffer, Document, image};
use tsumugi::controller::{TsumugiPortal, TsumugiPortalPlaneLocal, TsumugiControllerItemLifeTime, TsumugiControllerItemState, TsumugiControllerTrait, TsumugiObject};
use tsumugi::controller::TsumugiControllerItemState::Fulfilled;
use tsumugi::distributor::TsumugiParcelDistributor;
use tsumugi::parcel_receptor::TsumugiParcelReceptor;
use tsumugi::parcelreceptor_novalue::TsumugiParcelReceptorNoVal;

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
    ///Vec<u8>なのはVec<１頂点あたりのデータ>をVec<\[u8;バイト数\]>に変換しているから
    pub vertex:Vec<Vec<u8>>,
    pub index:Vec<Vec<u32>>
}

pub trait ObjectLoader {
    fn load()->Option<TsumugiVertexBinary>;
}


static TSUMUGI_STOCK_CPUNAME: &str = "TsumugiStockCPU";
///PathはオブジェクトのパスをカギとしたHashMap
#[derive(Clone)]
pub struct TsumugiStockController(pub Arc<Mutex<HashMap<&'static Path, TsumugiVertexBinary>>>);

///Pathはオブジェクトのパス
#[derive(Clone, Copy)]
pub struct TsumugiStock(pub &'static Path, pub fn() -> Option<TsumugiVertexBinary>);

impl TsumugiStockController {
    ///オブジェクトを保存するよ
    fn store(&self, stock_element: TsumugiStock, tc:&TsumugiPortalPlaneLocal) ->Option<&'static Path> {
        let thread_arc = self.clone();
        let stock = stock_element.clone();
        let thread_tc = tc.tp.clone();
        //todo:ここ雑にロードのマルチスレッド化を行っている
        thread::spawn(move ||{
            //ここでオブジェクトのロードがあるよ
            let figure_data = stock.1().unwrap();
            thread_arc.0.lock().unwrap().insert(stock.0, figure_data);
            Self::announce(stock_element.0,&thread_tc);
        });
        return Some(stock_element.0);
    }
    ///オブジェクトを保存して、バイナリを返すよ。
    fn load_store_sync(&self, stock_element: TsumugiStock) -> Option<TsumugiVertexBinary> {
        let material = stock_element.1()?;
        self.0.lock().unwrap().insert(stock_element.0, material.clone());
        Some(material)
    }
    ///オブジェクト引っ張ってくるよ、あったらそのまま返して無かったら生成して返すよ。
    fn load_sync(&self, path: TsumugiStock) -> Option<TsumugiVertexBinary> {
        if let Some(value) = self.0.lock().unwrap().get(path.0) {
            Some(value.clone())
        } else {
            self.load_store_sync(path)
        }
    }
    fn announce(figure_path:&'static Path,tc:&TsumugiPortal){
        //オブジェクトが生成されたら、生成されたことを「周知」させる
        tc.local_channel_sender.distributor_channel_sender.send(TsumugiParcelDistributor::new(figure_path).lifetime(TsumugiControllerItemLifeTime::Once).displayname("announce").into());
    }
}

impl Default for TsumugiStockController {
    fn default() -> Self {
        TsumugiStockController { 0: Arc::new(Mutex::new(Default::default())) }
    }
}
impl TsumugiObject for TsumugiStockController {
    fn on_create(&self, tc: &tsumugi::controller::TsumugiPortalPlaneLocal) {
        let mut object_hashmap = self.clone();
        let recept_object = TsumugiParcelReceptorNoVal::<TsumugiStock>::new()
            .subscribe_with_portal(Arc::new(move |object,tc| {
                //todo:ここキーをどうするか未定
                object_hashmap.store(*object.parcel.as_ref().unwrap().clone(),tc);
                TsumugiControllerItemState::Fulfilled
            })).to_antenna().displayname("recept_object");
        let dist_stock = TsumugiParcelDistributor::new(self.clone()).lifetime(TsumugiControllerItemLifeTime::Eternal).displayname("TsumugiStockController");
        tc.tp.local_channel_sender.distributor_channel_sender.send(dist_stock.into());
        tc.tp.local_channel_sender.recept_channel_sender.send(recept_object.into());
    }
}
impl TsumugiStock {
    pub fn store_figure(&self, tc: &TsumugiPortal) {
        let tsumugi_path = self.clone();
        let path_dist = TsumugiParcelDistributor::new(tsumugi_path);
        //todo:ここ生成が間に合わなくてパニックになるよ
        tc.find(TSUMUGI_STOCK_CPUNAME).unwrap().distributor_channel_sender.send(path_dist.into());
    }
}


pub fn spawn_figure_stock_handler(tc: &Box<TsumugiPortal>) -> Box<TsumugiPortal> {
    let mut newtc = tc.spawn(TSUMUGI_STOCK_CPUNAME.to_string());
    newtc.set_objects(vec![
        Box::new(TsumugiStockController::default()),
    ]);
    return newtc;
}
