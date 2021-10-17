use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use gltf::{buffer, Document, image};
use tsumugi::controller::{TsumugiController, TsumugiControllerItemState, TsumugiControllerTrait, TsumugiObject};
use tsumugi::distributor::TsumugiParcelDistributor;
use tsumugi::parcel_receptor::TsumugiParcelReceptor;

type Import = (Document, Vec<buffer::Data>, Vec<image::Data>);

struct tsumugiStock();

static tsumugiStockCPUName: &str = "TsumugiStockCPU";

#[derive(Clone)]
struct TsumugiStockController(Arc<Mutex<HashMap<&'static Path, Arc<Import>>>>);

#[derive(Clone, Copy)]
pub struct TsumugiStock(pub &'static Path);

impl TsumugiStockController {
    fn store(&self, path: TsumugiStock) {
        let import = gltf::import(path.0).unwrap_or_else(|x| { panic!("{}", x) });
        self.0.lock().unwrap().insert(path.0, Arc::from(import));
    }
    fn load_store_sync(&self, path: TsumugiStock) -> Arc<Import> {
        let import = gltf::import(path.0).unwrap_or_else(|x| { panic!("{}", x) });
        let arc_import = Arc::new(import);
        self.0.lock().unwrap().insert(path.0, arc_import.clone());
        arc_import
    }
    fn load_sync(&self, path: TsumugiStock) -> Arc<Import> {
        if let Some(value) = self.0.lock().unwrap().get(path.0) {
            value.clone()
        } else {
            self.load_store_sync(path)
        }
    }
}

impl Default for TsumugiStockController {
    fn default() -> Self {
        TsumugiStockController { 0: Arc::new(Mutex::new(Default::default())) }
    }
}

impl TsumugiObject for TsumugiStockController {
    fn on_create(&self, tc: &tsumugi::controller::TsumugiController_thread) {
        let mut object_hashmap = self.clone();
        let recept_object = TsumugiParcelReceptor::new(TsumugiStock { 0: &Path::new("") })
            .subscribe(Arc::new(move |object| {
                let object_key_distribution = object_hashmap.store(*object.parcel);
                TsumugiControllerItemState::Fulfilled
            })).to_antenna();
        tc.tc.local_channel_sender.recept_channel_sender.send(recept_object.into());
    }
}

impl TsumugiStock {
    pub fn store_object(&self, tc: &TsumugiController) {
        let tsumugi_path = self.clone();
        let path_dist = TsumugiParcelDistributor::new(tsumugi_path);
        tc.global_connect_tsumugi_controller.lock().unwrap().get(tsumugiStockCPUName).unwrap().local_channel_sender.pickup_channel_sender.send(path_dist.into());
    }
}


pub fn spown_object_stock_handler(tc: &Box<TsumugiController>) -> Box<TsumugiController> {
    let mut newtc = tc.spown(tsumugiStockCPUName.to_string());
    newtc.set_objects(vec![
        Box::new(TsumugiStockController::default()),
    ]);
    return newtc;
}
