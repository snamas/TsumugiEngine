pub mod camera;

use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicBool, AtomicU64};
use std::sync::atomic::Ordering::SeqCst;
use nalgebra::{Point, Point2, Point3, Translation, Translation3};
use tsumuFigureStockCPU::TsumugiVertexBinary;
use tsumugi::controller::{TsumugiPortal, TsumugiPortalPlaneLocal, TsumugiControllerItemState, TsumugiControllerTrait, TsumugiObject, TsumugiControllerItemLifeTime};
use tsumugi::controller::TsumugiControllerItemLifeTime::{Eternal, Once};
use tsumugi::distributor::TsumugiParcelDistributor;
use tsumugi::parcel_receptor::TsumugiParcelReceptor;
use tsumugi::parcelreceptor_novalue::TsumugiParcelReceptorNoVal;
use tsumuFigureStockCPU::{TsumugiStock};
use tsumugi::controller::TsumugiControllerItemState::Fulfilled;
use crate::camera::Camera;

static CONTROLLER_NAME: &str = "tsumugi3dObject";
///ObjectKeyはオブジェクト固有の番号。すべてのオブジェクト更新にはこの鍵が必要
#[derive(Clone)]
pub struct ObjectKey(Arc<RwLock<Option<u64>>>);

#[derive(Clone)]
pub struct TsumugiObjectController {
    pub object_hashmap: Arc<Mutex<HashMap<u64, Tsumugi3DObject>>>,
    object_key_origin: Arc<AtomicU64>,
}

#[derive(Clone)]
struct TsumugiObjectConstructor{
    object_id:u64,
    object:Tsumugi3DObject
}

#[derive(Copy, Clone)]
enum Tsumugi3DObjectAction {
    Crate,
    Update,
    Delete,
}

#[derive(Copy, Clone)]
pub struct Tsumugi3DObject {
    position: nalgebra::Point3<f32>,
    rotate: nalgebra::Point2<f32>,
    size: nalgebra::Point3<f32>,
    pub name: &'static str,
    pub figure_data_path: &'static Path,
    object_load_function: fn() -> Option<TsumugiVertexBinary>,
    ///マテリアルの名前。
    pub material_name: &'static str,
    material_element_id: u64,
}

#[derive(Clone)]
struct Tsumugi3DObjectParcel {
    tsumugi3dobject: Tsumugi3DObject,
    object_key: ObjectKey,
    tsumugi3dobject_action: Tsumugi3DObjectAction,
}


impl TsumugiObjectController {
    pub fn new() -> Self {
        TsumugiObjectController { object_hashmap: Arc::new(Mutex::new(HashMap::new())), object_key_origin: Arc::new(AtomicU64::new(0)) }
    }
    ///オブジェクトをobject_hashmapの中に格納するよ。格納したら固有の番号が取り出されるよ。この番号を使ってupdateでアクセスするよ。懸念事項：u64の数字をすべて使い切るとフリーズする。
    pub fn insert(&self, obj: Tsumugi3DObject) -> u64 {
        self.object_key_origin.fetch_add(1, SeqCst);
        if let std::collections::hash_map::Entry::Vacant(e) = self.object_hashmap.lock().unwrap().entry(self.object_key_origin.load(SeqCst)) {
            e.insert(obj);
            return self.object_key_origin.load(SeqCst);
        }
        self.insert(obj)
    }
    pub fn update(&self, obj: Tsumugi3DObject, object_key: u64) {
        self.object_hashmap.lock().unwrap().insert(object_key, obj);
    }
    pub fn delete(&self, object_key: u64) {
        self.object_hashmap.lock().unwrap().remove(&object_key);
    }
}

///外部使用想定の構造体、オブジェクトを管理するよ
impl Tsumugi3DObject {
    ///新しく作るよ
    pub fn new(name: &'static str, path: &'static Path,material:&'static str, object_load_function: fn() -> Option<TsumugiVertexBinary>) -> Self {
        Tsumugi3DObject {
            position: Point3::new(0.0, 0.0, 0.0),
            rotate: Point2::new(0.0, 0.0),
            size: Point3::new(0.0, 0.0, 0.0),
            name,
            figure_data_path: path,
            object_load_function: object_load_function,
            material_name: material,
            material_element_id: 0
        }
    }
    ///tsumugi3dObjectプレーンに送るよ。今後updateで使う鍵が返ってくるよ
    pub fn create3d_object(&self, tc: &TsumugiPortal) -> ObjectKey {
        let mut tsumugi3dobject_parcel = Tsumugi3DObjectParcel {
            tsumugi3dobject: self.clone(),
            object_key: ObjectKey(Arc::new(RwLock::new(None))),
            tsumugi3dobject_action: Tsumugi3DObjectAction::Crate
        };
        {
            let tsumugiStock = TsumugiStock { 0: tsumugi3dobject_parcel.tsumugi3dobject.figure_data_path, 1: self.object_load_function };
            tsumugiStock.store_figure(tc);
        }
        let duplicate_key = tsumugi3dobject_parcel.object_key.clone();
        let p_dist = TsumugiParcelDistributor::new(tsumugi3dobject_parcel);
        tc.find(CONTROLLER_NAME).unwrap().distributor_channel_sender.send(p_dist.into());
        duplicate_key
    }
    ///tsumugi3dObjectプレーンの値を変えるよ。
    pub fn update3d_object(&self, tc: &TsumugiPortal, object_key: ObjectKey) -> bool {
        if object_key.0.read().unwrap().is_some() {
            let tsumugi3dobject_parcel = Tsumugi3DObjectParcel {
                tsumugi3dobject: self.clone(),
                object_key: object_key,
                tsumugi3dobject_action: Tsumugi3DObjectAction::Update,
            };
            let p_dist = TsumugiParcelDistributor::new(tsumugi3dobject_parcel);
            tc.find(CONTROLLER_NAME).unwrap().distributor_channel_sender.send(p_dist.into());
            return true;
        }
        return false;
    }
    ///tsumugi3dObjectを消すよ。準備中
    pub fn delete3d_object(&self){
        todo!("あとで実装するよ")
    }
}

impl TsumugiObject for TsumugiObjectController {
    fn on_create(&self, tc: &TsumugiPortalPlaneLocal) {
        let mut object_hashmap = self.clone();
        let recept_object = TsumugiParcelReceptorNoVal::<Tsumugi3DObjectParcel>::new()
            .subscribe_with_portal(Arc::new(move |object, tp| {
                let object = object.parcel.clone().unwrap();
                match &object.tsumugi3dobject_action {
                    Tsumugi3DObjectAction::Crate => {
                        ///受け取ったオブジェクトを格納するよ。
                        ///その後、そのオブジェクトの管理番号を返すよ。
                        let object_key_distribution = object_hashmap.insert(object.tsumugi3dobject);
                        object.object_key.0.write().unwrap().replace(object_key_distribution);
                        tp.tp.local_channel_sender.distributor_channel_sender
                            .send(TsumugiParcelDistributor::new(TsumugiObjectConstructor{ object_id: object_key_distribution, object: object.tsumugi3dobject })
                                .displayname("Object_Spawn!")
                                .lifetime(TsumugiControllerItemLifeTime::Once)
                                .parcelname("object_spawn")
                                .into());
                    }
                    //todo:ここobject_spawnするときにCreateとUpdateの区別をしていないよ
                    Tsumugi3DObjectAction::Update => {
                        if let Some(key) = *object.object_key.0.read().unwrap(){
                            object_hashmap.update(object.tsumugi3dobject,key);
                            tp.tp.local_channel_sender.distributor_channel_sender
                                .send(TsumugiParcelDistributor::new(TsumugiObjectConstructor{ object_id: key, object: object.tsumugi3dobject })
                                    .displayname("Object_Spawn!")
                                    .lifetime(TsumugiControllerItemLifeTime::Once)
                                    .parcelname("object_spawn")
                                    .into());
                        }
                    }
                    Tsumugi3DObjectAction::Delete => {
                        todo!("ここはまだ出来てないよ")
                    }
                }
                TsumugiControllerItemState::Fulfilled
            })).to_antenna().displayname("recept_object");
        tc.tp.local_channel_sender.recept_channel_sender.send(recept_object.into());
        let dist_object = TsumugiParcelDistributor::new(self.clone()).displayname("object_distributor").lifetime(Eternal);
        tc.tp.local_channel_sender.distributor_channel_sender.send(dist_object.into());
    }
}
///第一引数にポータルの参照、第二引数にオブジェクトを受け取ったらどうするかが入るよ。
fn fetch_3dobject(tp:&TsumugiPortal,fetch_func:fn(&u64,&Tsumugi3DObject)){
    let first_fetch = TsumugiParcelReceptorNoVal::<TsumugiObjectController>::new().subscribe(Arc::new(move |object_list|{
        object_list.parcel.as_ref().unwrap().object_hashmap.lock().unwrap().iter().for_each(|(object_key,object)|{
            fetch_func(object_key,object);
        });
        Fulfilled
    })).to_antenna().displayname("Object_FirstGet!").parcelname("object_spawn").lifetime(Once);
    let connect_fetch = TsumugiParcelReceptorNoVal::<TsumugiObjectConstructor>::new().subscribe(Arc::new(move |object|{
        let key = object.parcel.as_ref().unwrap().object_id;
        fetch_func(&key, &object.parcel.as_ref().unwrap().object);
        Fulfilled
    })).to_antenna().displayname("Object_Get!").parcelname("object_spawn").lifetime(Eternal);
    tp.find(CONTROLLER_NAME).unwrap().recept_channel_sender.send(first_fetch.into());
    tp.find(CONTROLLER_NAME).unwrap().recept_channel_sender.send(connect_fetch.into());
}

pub fn spawn_3d_object_handler(tc: &Box<TsumugiPortal>) -> Box<TsumugiPortal> {
    let mut newtc = tc.spawn(CONTROLLER_NAME.to_string());
    newtc.set_objects(vec![
        Box::new(TsumugiObjectController { object_hashmap: Default::default(), object_key_origin: Arc::new(AtomicU64::new(0)) }),
        Box::new(Camera::new(Translation3::new(0f32,0f32,-3f32), Default::default()))
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