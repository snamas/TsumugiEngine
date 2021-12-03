use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicBool, AtomicU64};
use std::sync::atomic::Ordering::SeqCst;
use nalgebra::{Point, Point2, Point3};
use tsumuFigureStockCPU::TsumugiVertexBinary;
use tsumugi::controller::{TsumugiController, TsumugiController_threadlocal, TsumugiControllerItemState, TsumugiControllerTrait, TsumugiObject};
use tsumugi::controller::TsumugiControllerItemLifeTime::Eternal;
use tsumugi::distributor::TsumugiParcelDistributor;
use tsumugi::parcel_receptor::TsumugiParcelReceptor;
use tsumugi::parcelreceptor_novalue::TsumugiParcelReceptorNoVal;
use tsumuFigureStockCPU::{TsumugiStock};

static CONTROLLER_NAME: &str = "tsumugi3dObject";
///ObjectKeyはオブジェクト固有の番号。すべてのオブジェクト更新にはこの鍵が必要
#[derive(Clone)]
pub struct ObjectKey(Arc<RwLock<Option<u64>>>);

#[derive(Clone)]
pub struct TsumugiObjectController {
    pub object_hashmap: Arc<Mutex<HashMap<u64, Tsumugi3DObject>>>,
    object_key_origin: Arc<AtomicU64>,
}

struct TsumugiObjectConstructor();

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

///外部使用想定
impl Tsumugi3DObject {
    ///新しく作るよ
    pub fn new(name: &'static str, path: &'static Path, object_load_function: fn() -> Option<TsumugiVertexBinary>) -> Self {
        Tsumugi3DObject {
            position: Point3::new(0.0, 0.0, 0.0),
            rotate: Point2::new(0.0, 0.0),
            size: Point3::new(0.0, 0.0, 0.0),
            name,
            figure_data_path: path,
            object_load_function: object_load_function,
        }
    }
    ///tsumugi3dObjectプレーンに送るよ
    pub fn create3d_object(&self, tc: &TsumugiController) -> ObjectKey {
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
        tc.find(CONTROLLER_NAME).unwrap().pickup_channel_sender.send(p_dist.into());
        duplicate_key
    }
    ///tsumugi3dObjectプレーンの値を変えるよ。今のところ準備中
    pub fn update3d_object(self, tc: &TsumugiController, object_key: ObjectKey) -> bool {
        if object_key.0.read().unwrap().is_some() {
            let tsumugi3dobject_parcel = Tsumugi3DObjectParcel {
                tsumugi3dobject: self,
                object_key: object_key,
                tsumugi3dobject_action: Tsumugi3DObjectAction::Update,
            };
            let p_dist = TsumugiParcelDistributor::new(tsumugi3dobject_parcel);
            tc.find(CONTROLLER_NAME).unwrap().pickup_channel_sender.send(p_dist.into());
            return true;
        }
        return false;
    }
}

impl TsumugiObject for TsumugiObjectController {
    fn on_create(&self, tc: &TsumugiController_threadlocal) {
        let mut object_hashmap = self.clone();
        let recept_object = TsumugiParcelReceptorNoVal::<Tsumugi3DObjectParcel>::new()
            .subscribe(Arc::new(move |object| {
                ///受け取ったオブジェクトを格納するよ。
                ///その後、そのオブジェクトの管理番号を返すよ。
                let objectClone = object.parcel.clone().unwrap();
                let object_key_distribution = object_hashmap.insert(objectClone.tsumugi3dobject.clone());
                objectClone.object_key.0.write().unwrap().replace(object_key_distribution);
                TsumugiControllerItemState::Fulfilled
            })).to_antenna().displayname("recept_object");
        tc.tc.local_channel_sender.recept_channel_sender.send(recept_object.into());
        let dist_object = TsumugiParcelDistributor::new(self.clone()).displayname("object_distributor").lifetime(Eternal);
        tc.tc.local_channel_sender.pickup_channel_sender.send(dist_object.into());
    }
}

pub fn spown_3d_object_handler(tc: &Box<TsumugiController>) -> Box<TsumugiController> {
    let mut newtc = tc.spown(CONTROLLER_NAME.to_string());
    newtc.set_objects(vec![
        Box::new(TsumugiObjectController { object_hashmap: Default::default(), object_key_origin: Arc::new(AtomicU64::new(0)) }),
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