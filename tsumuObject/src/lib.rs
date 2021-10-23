use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU64};
use std::sync::atomic::Ordering::SeqCst;
use nalgebra::{Point, Point2, Point3};
use tsumugi::controller::{TsumugiController, TsumugiController_thread, TsumugiControllerItemState, TsumugiControllerTrait, TsumugiObject};
use tsumugi::controller::TsumugiControllerItemLifeTime::Eternal;
use tsumugi::distributor::TsumugiParcelDistributor;
use tsumugi::parcel_receptor::TsumugiParcelReceptor;
use tsumuStockCPU::{TsumugiStock};
static Controller_name:&str = "tsumugi3dObject";

#[derive(Clone)]
pub struct ObjectKey(Arc<Mutex<Option<u64>>>);
#[derive( Clone)]
pub struct TsumugiObjectController{pub object_hashmap:Arc<Mutex<HashMap<u64,Tsumugi3DObject>>>, object_key_origin:Arc<AtomicU64>}
struct TsumugiObjectConstructor();
#[derive(Copy, Clone)]
enum Tsumugi3DObjectAction{
    Crate,
    Update,
    Delete
}
#[derive(Copy, Clone)]
pub struct Tsumugi3DObject{
    position:nalgebra::Point3<f32>,
    rotate:nalgebra::Point2<f32>,
    size:nalgebra::Point3<f32>,
    pub name:&'static str,
    pub path:&'static Path
}
#[derive(Clone)]
struct Tsumugi3DObjectParcel{
    tsumugi3dobject:Tsumugi3DObject,
    object_key:ObjectKey,
    tsumugi3dobject_action:Tsumugi3DObjectAction
}

impl TsumugiObjectController {
    pub fn new()->Self{
        TsumugiObjectController{object_hashmap:Arc::new(Mutex::new(HashMap::new())), object_key_origin:Arc::new(AtomicU64::new(0))}
    }
    pub fn insert(&self, obj:Tsumugi3DObject) -> u64 {
        self.object_key_origin.fetch_add(1,SeqCst);
        if let std::collections::hash_map::Entry::Vacant(e) = self.object_hashmap.lock().unwrap().entry(self.object_key_origin.load(SeqCst)) {
            e.insert(obj);
            return self.object_key_origin.load(SeqCst);
        }
        self.insert(obj)
    }
    pub fn update(&self,obj:Tsumugi3DObject,object_key:u64){
        self.object_hashmap.lock().unwrap().insert(object_key,obj);
    }
    pub fn delete(&self,object_key:u64){
        self.object_hashmap.lock().unwrap().remove(&object_key);
    }
}
impl Default for Tsumugi3DObject{
    fn default() -> Self {
        Tsumugi3DObject{
            position: Point3::new(0.0,0.0,0.0),
            rotate: Point2::new(0.0,0.0),
            size: Point3::new(0.0,0.0,0.0),
            name: "",
            path: &Path::new("")
        }
    }
}

impl Default for Tsumugi3DObjectParcel{
    fn default() -> Self {
        Tsumugi3DObjectParcel{
            tsumugi3dobject: Default::default(),
            object_key: ObjectKey(Arc::new(Mutex::new(None))),
            tsumugi3dobject_action: Tsumugi3DObjectAction::Crate
        }
    }
}

impl Tsumugi3DObject {
    pub fn new(name:&'static str,path:&'static Path)->Self{
        Tsumugi3DObject {
            name,
            path,
            ..Default::default()
        }
    }
    pub fn create3d_object(&self,tc: &TsumugiController)->ObjectKey{
        let mut tsumugi3dobject_parcel = Tsumugi3DObjectParcel::default();
        tsumugi3dobject_parcel.tsumugi3dobject = self.clone();
        let tsumugiStock = TsumugiStock{ 0: tsumugi3dobject_parcel.tsumugi3dobject.path };
        let duplicate_key = tsumugi3dobject_parcel.object_key.clone();
        let p_dist = TsumugiParcelDistributor::new(tsumugi3dobject_parcel);
        tsumugiStock.store_object(tc);
        tc.find(Controller_name).pickup_channel_sender.send(p_dist.into());
        duplicate_key
    }
    pub fn update3d_object(self,tc: &TsumugiController,object_key:ObjectKey)->bool{
        if object_key.0.lock().unwrap().is_some(){
            let tsumugi3dobject_parcel = Tsumugi3DObjectParcel{
                tsumugi3dobject: self,
                object_key: object_key,
                tsumugi3dobject_action: Tsumugi3DObjectAction::Update
            };
            let p_dist = TsumugiParcelDistributor::new(tsumugi3dobject_parcel);
            tc.find(Controller_name).pickup_channel_sender.send(p_dist.into());
            return true;
        }
        return false
    }
}
impl TsumugiObject for TsumugiObjectController{
    fn on_create(&self, tc: &TsumugiController_thread) {
        let mut object_hashmap = self.clone();
        let recept_object = TsumugiParcelReceptor::new(Tsumugi3DObjectParcel::default())
            .subscribe(Arc::new(move |object|{
                let object_key_distribution = object_hashmap.insert(object.parcel.tsumugi3dobject.clone());
                object.parcel.object_key.0.lock().unwrap().replace(object_key_distribution);
                TsumugiControllerItemState::Fulfilled
            })).to_antenna();
        tc.tc.local_channel_sender.recept_channel_sender.send(recept_object.into());
        let dist_object = TsumugiParcelDistributor::new(self.clone()).lifetime(Eternal);
        tc.tc.local_channel_sender.pickup_channel_sender.send(dist_object.into());
    }
}

pub fn spown_3d_object_handler(tc: &Box<TsumugiController>) -> Box<TsumugiController> {
    let mut newtc = tc.spown(Controller_name.to_string());
    newtc.set_objects(vec![
        Box::new(TsumugiObjectController{ object_hashmap: Default::default(), object_key_origin: Arc::new(AtomicU64::new(0)) }),
    ]);
    return newtc;
}

#[cfg(test)]
mod tests {

    #[test]
    fn object_create() {
        //todo!();
    }

}