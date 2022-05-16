use std::any::{TypeId, Any};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{mpsc, Arc, Mutex, Condvar};
use std::thread;
use std::thread::{JoinHandle, sleep};
use std::collections::HashMap;
use crate::antenna::{TsumugiAntenna, TsumugiFuture};
use crate::distributor::{TsumugiParcelDistributor, TsumugiDistributor};
use crate::antenna_chain::{TsumugiAntennaChain, TsumugiAntennaType, TsumugiAntennaChainType};
use std::time::{Instant, Duration};
use crate::signal::TsumugiSignal;
use crate::controller::TsumugiControllerItemLifeTime::Once;

/// Controllerに送る際、どう作成するか
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TsumugiControllerApplication {
    /// 新規作成
    New,
    /// LifeTimeの型が同じものはすべて破棄され、このアンテナに更新される。LifeCount(u32)などに存在する値は考慮しない。（更新対象が存在しない場合、新規作成される）
    Renew,
    /// 更新する（更新対象が存在しない場合、新規作成されず、棄却される）
    RenewOrReject,
    /// 破棄する（LifeTimeの型が同じものはすべて破棄される）
    Drop,
}

/// 扱うものの現在の状況を示すenum
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TsumugiControllerItemState {
    /// 未処理
    Untreated,
    /// 処理が棄却された
    Deny,
    /// 処理が受理された
    Fulfilled,
    /// 処理中
    OnProgress,
    /// この処理が終わった時に、消去する。（アンテナ限定）
    /// todo:Eliminateの実装を詰める。一度Eliminateしたが後続がFulfilledした場合に無効になる可能性があるため。
    Eliminate,
}

/// 生存期間を示すenum
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TsumugiControllerItemLifeTime {
    /// Ⅰサイクルのみ生存する
    Flash,
    /// 一度値を受け取るまで生存する
    Once,
    ///永遠に生存する
    Eternal,
    ///一定時間のみ生存する
    Lifetime(std::time::Duration),
    ///決めた回数値を受け取るまで生存する
    LifeCount(u32),
    ///決めたサイクル数まで生存する
    Lifecycle(u32),
}
///つむぎ内で使う構造体。つむぎの外や、クローンする場合はTsumugiPortalを使おう。
pub struct TsumugiPortalPlaneLocal {
    pub tp: TsumugiPortal,
    pub(crate) tsumugi_object_vector: Vec<Box<dyn TsumugiObject + Send>>,
}

#[derive(Clone)]
pub struct TsumugiPortal {
    pub local_channel_sender: TsumugiChannelSenders,
    pub global_channel_sender: TsumugiChannelSenders,
    pub(crate) connect_tsumugi_controller: Vec<String>,
    pub global_connect_tsumugi_controller: Arc<Mutex<HashMap<String, Box<TsumugiPortal>>>>,
    pub tsumugi_controller_name: String,
    pub(crate) tsumugi_object_sender: Sender<Box<dyn TsumugiObject + Send>>,
}

pub struct ThreadReceivers {
    pub(crate) distributer: Receiver<TsumugiDistributor>,
    pub(crate) antenna: Receiver<TsumugiAntennaType>,
    pub(crate) object: Receiver<Box<dyn TsumugiObject + Send>>,
}

#[derive(Clone)]
pub struct TsumugiChannelSenders {
    pub pickup_channel_sender: Sender<TsumugiDistributor>,
    pub recept_channel_sender: Sender<TsumugiAntennaType>,
}

pub struct TsumugiParcelHashList {
    pub(crate) pickup_list_withid: HashMap<String, Vec<TsumugiParcelDistributor>>,
    pub(crate) recept_list_withid: HashMap<String, Vec<TsumugiAntenna>>,
    pub(crate) pickup_list: Vec<TsumugiParcelDistributor>,
    pub(crate) recept_list: Vec<TsumugiAntenna>,
}

pub struct TsumugiSignalHashList {
    pickup_list: Vec<TsumugiSignal>,
    recept_list: Vec<TsumugiSignal>,
}

pub(crate) struct DepotHashList {
    pub(crate) antenna_hashmap: HashMap<TypeId, TsumugiParcelHashList>,
    pub(crate) signal_hashmap: HashMap<String, TsumugiSignalHashList>,
}

pub struct TsumugiAntennaChainHashList {
    pub(crate) receipt_list: Vec<TsumugiAntennaChain>,
}

pub(crate) struct ControllLoopKitStruct {
    pub(crate) thread_receivers: ThreadReceivers,
    pub(crate) inst_time: Instant,
    pub(crate) depot_hashmap_typeof: DepotHashList,
    pub(crate) antennachain_hashmap: TsumugiAntennaChainHashList,
    pub(crate) object_list: Vec<Box<dyn TsumugiObject + Send>>,
}

impl TsumugiControllerItemLifeTime {
    pub fn compare(&self, antenna: &TsumugiControllerItemLifeTime) -> bool {
        match (self, antenna) {
            (TsumugiControllerItemLifeTime::Flash, TsumugiControllerItemLifeTime::Flash) => { true }
            (TsumugiControllerItemLifeTime::Once, TsumugiControllerItemLifeTime::Once) => { true }
            (TsumugiControllerItemLifeTime::Eternal, TsumugiControllerItemLifeTime::Eternal) => { true }
            (TsumugiControllerItemLifeTime::Lifetime(_), TsumugiControllerItemLifeTime::Lifetime(_)) => { true }
            (TsumugiControllerItemLifeTime::LifeCount(_), TsumugiControllerItemLifeTime::LifeCount(_)) => { true }
            (TsumugiControllerItemLifeTime::Lifecycle(_), TsumugiControllerItemLifeTime::Lifecycle(_)) => { true }
            _ => { false }
        }
    }
}

pub trait TsumugiObject {
    fn on_create(&self, tc: &TsumugiPortalPlaneLocal);
}

pub trait TsumugiControllerTrait {
    fn new(tsumuginame: String) -> Box<TsumugiPortal>;
    fn spown(self: &Box<Self>, tsumuginame: String) -> Box<TsumugiPortal>;
    fn set_object(&mut self, tsumugi_object:Box<dyn TsumugiObject + Send>);
    fn set_objects(&mut self, tsumugi_object_list: Vec<Box<dyn TsumugiObject + Send>>);
    fn find(&self, Controller_name: &str) -> Option<TsumugiChannelSenders>;
    fn execute_tsumugi_functions(self: &Box<Self>, tsumugi_functions: Vec<fn(&Box<TsumugiPortal>) -> Box<TsumugiPortal>>);
    fn execute_tsumugi_thread(&self, thread_receivers: ThreadReceivers) -> JoinHandle<()>;
}

impl TsumugiControllerTrait for TsumugiPortal {
    //todo:box化する意味はあんまりない気がする。
    //todo:newとspownでコードの一部がかぶっているのをなんとかしたい
    fn new(tsumuginame: String) -> Box<TsumugiPortal> {
        let (recept_channel_sender, receipt_channnel_receiver) = mpsc::channel();
        let (pickup_channel_sender, pickup_channnel_receiver) = mpsc::channel();
        let (object_sender, object_receiver) = mpsc::channel();
        let tsumugi_channel_senders = TsumugiChannelSenders { pickup_channel_sender, recept_channel_sender };
        let mut tsumugi_connect_list: Vec<String> = Vec::new();
        let mut tc = Box::new(TsumugiPortal {
            local_channel_sender: tsumugi_channel_senders.clone(),
            global_channel_sender: tsumugi_channel_senders,
            connect_tsumugi_controller: tsumugi_connect_list,
            global_connect_tsumugi_controller: Arc::new(Mutex::new(HashMap::new())),
            tsumugi_controller_name: tsumuginame,
            tsumugi_object_sender: object_sender,
        });
        let receivers = ThreadReceivers {
            distributer: pickup_channnel_receiver,
            antenna: receipt_channnel_receiver,
            object: object_receiver,
        };
        tc.execute_tsumugi_thread(receivers);
        return tc;
    }

    fn spown(self: &Box<Self>, tsumuginame: String) -> Box<TsumugiPortal> {
        let (recept_channel_sender, receipt_channnel_receiver) = mpsc::channel();
        let (pickup_channel_sender, pickup_channnel_receiver) = mpsc::channel();
        let (object_sender, object_receiver) = mpsc::channel();
        let tsumugi_channel_senders = TsumugiChannelSenders { pickup_channel_sender, recept_channel_sender };
        let mut tsumugi_connect_list: Vec<String> = Vec::new();
        let mut tc = Box::new(TsumugiPortal {
            local_channel_sender: tsumugi_channel_senders.clone(),
            global_channel_sender: self.global_channel_sender.clone(),
            connect_tsumugi_controller: tsumugi_connect_list,
            global_connect_tsumugi_controller: self.global_connect_tsumugi_controller.clone(),
            tsumugi_controller_name: tsumuginame,
            tsumugi_object_sender: object_sender,
        });
        let receivers = ThreadReceivers {
            distributer: pickup_channnel_receiver,
            antenna: receipt_channnel_receiver,
            object: object_receiver,
        };
        tc.execute_tsumugi_thread(receivers);
        return tc;
    }
    fn set_object(&mut self, mut tsumugi_object:Box<dyn TsumugiObject + Send>){
        self.tsumugi_object_sender.send(tsumugi_object);
    }
    // todo:ここ、すべてのスレッドを起動してから送りたい。そうでないと落ちる場合がある。
    fn set_objects(&mut self, mut tsumugi_object_list: Vec<Box<dyn TsumugiObject + Send>>) {
        for tsumugi_object in tsumugi_object_list {
            self.tsumugi_object_sender.send(tsumugi_object);
        }
    }
    fn find(&self, Controller_name: &str) -> Option<TsumugiChannelSenders> {
        Some(self.global_connect_tsumugi_controller.lock().unwrap().get(Controller_name)?.local_channel_sender.clone())
    }
    /// TsumugiController生成関数の配列を受け取って,それを使ってTsumugiControllerを生成していくよ
    fn execute_tsumugi_functions(self: &Box<Self>, create_tsumugi_controller_funclist: Vec<fn(&Box<TsumugiPortal>) -> Box<TsumugiPortal>>) {
        for tsumugi_function in create_tsumugi_controller_funclist {
            let mut tc_new = tsumugi_function(self);
            self.global_connect_tsumugi_controller.lock().unwrap().insert(tc_new.tsumugi_controller_name.clone(), tc_new as Box<TsumugiPortal>);
        }
    }
    fn execute_tsumugi_thread(&self, thread_receivers: ThreadReceivers) -> JoinHandle<()> {
        let mut tc_thread = TsumugiPortalPlaneLocal { tp: self.clone(), tsumugi_object_vector: vec![] };
        let plane_name = self.tsumugi_controller_name.clone();
        thread::spawn(move || {
            //todo:うまいことロックを使いこなそうcondvarというやつをつかって
            //todo:あとhashを値の管理に使う。
            let mut tumugi_receipt_list: Vec<Box<dyn TsumugiFuture + Send>> = Vec::new();
            let mut pickup_list: Vec<Box<dyn Any + Send>> = Vec::new();
            let condvar = Condvar::new();
            let mutex = Mutex::new(());
            let lock = mutex.lock().unwrap();
            //condvar.wait(lock).unwrap();
            let mut depot_hashmap_typeof = DepotHashList { antenna_hashmap: HashMap::new(), signal_hashmap: HashMap::new() };
            let mut antennachain_hashmap = TsumugiAntennaChainHashList { receipt_list: Vec::new() };
            let mut inst_time = Instant::now();
            let mut controll_loop_kit = ControllLoopKitStruct {
                thread_receivers,
                inst_time,
                depot_hashmap_typeof,
                antennachain_hashmap,
                object_list: vec![],
            };
            loop {
                //todo:とりあえず１msごとに更新する
                sleep(Duration::new(0, 1));
                tc_thread.thread_loop_antenna_parcel(&mut controll_loop_kit);
                if let Some(debug_plane) = tc_thread.tp.find("TsumugiDebugWin"){
                    let debugkit = controll_loop_kit.debug(plane_name.clone());
                    debug_plane.pickup_channel_sender.send(TsumugiParcelDistributor::new(debugkit).lifetime(Once).displayname(plane_name.clone()).into());
                }
            }
        }
        )
    }
}

impl TsumugiPortalPlaneLocal {
    fn set_objects(&mut self, mut tsumugi_object_list: Vec<Box<dyn TsumugiObject + Send>>) {
        for tsumugi_object in &tsumugi_object_list {
            tsumugi_object.on_create(self);
        }
        self.tsumugi_object_vector.append(&mut tsumugi_object_list);
    }
    fn set_object(&mut self, mut tsumugi_object: Box<dyn TsumugiObject + Send>) {
        tsumugi_object.on_create(self);
        self.tsumugi_object_vector.push(tsumugi_object);
    }
    pub(crate) fn thread_loop_antenna_parcel(&mut self, controll_loop_kit: &mut ControllLoopKitStruct) {
        let mut pickup_iter = controll_loop_kit.thread_receivers.distributer.try_iter();
        let mut receipt_iter = controll_loop_kit.thread_receivers.antenna.try_iter();
        for pickup_item in pickup_iter {
            match pickup_item {
                TsumugiDistributor::TPDistributor(parcel) => { self.parcel_action(parcel, &mut controll_loop_kit.depot_hashmap_typeof); }
                TsumugiDistributor::TsumugiSignal(signal) => {
                    let mut hashlist = controll_loop_kit.depot_hashmap_typeof.signal_hashmap.entry(signal.signal_name.clone()).or_insert(TsumugiSignalHashList { pickup_list: vec![], recept_list: vec![] });
                    hashlist.pickup_list.push(signal);
                }
            }
        }
        for receive_item in receipt_iter {
            self.antenna_chain_action(receive_item, &mut controll_loop_kit.depot_hashmap_typeof, &mut controll_loop_kit.antennachain_hashmap);
        }
        self.set_objects(controll_loop_kit.thread_receivers.object.try_iter().collect());

        self.signal_loop(controll_loop_kit);
        self.antenna_chain_loop(controll_loop_kit);
        for tsumugi_hash in controll_loop_kit.depot_hashmap_typeof.antenna_hashmap.iter_mut() {
            for antenna in &mut tsumugi_hash.1.recept_list {
                match antenna.antennalifetime {
                    TsumugiControllerItemLifeTime::Lifecycle(mut x) => {
                        x = x.saturating_sub(1);
                        antenna.antennalifetime = TsumugiControllerItemLifeTime::Lifecycle(x);
                    }
                    TsumugiControllerItemLifeTime::Lifetime(mut x) => {
                        x = x.saturating_sub(controll_loop_kit.inst_time.elapsed());
                        antenna.antennalifetime = TsumugiControllerItemLifeTime::Lifetime(x);
                    }
                    _ => {}
                }
            }
            tsumugi_hash.1.recept_list.retain(|val| {
                match (val.antennalifetime, val.current_state) {
                    (TsumugiControllerItemLifeTime::Flash, _) => false,
                    (TsumugiControllerItemLifeTime::Once, TsumugiControllerItemState::Fulfilled) => false,
                    (TsumugiControllerItemLifeTime::Lifecycle(0), _) => false,
                    (TsumugiControllerItemLifeTime::LifeCount(0), _) => false,
                    (TsumugiControllerItemLifeTime::Lifetime(Duration::ZERO), _) => false,
                    (_, TsumugiControllerItemState::Eliminate) => false,
                    (_, _) => true
                }
            });
            for parcel in &mut tsumugi_hash.1.pickup_list {
                match parcel.parcellifetime {
                    TsumugiControllerItemLifeTime::Lifetime(mut x) => {
                        x = x.saturating_sub(controll_loop_kit.inst_time.elapsed());
                        parcel.parcellifetime = TsumugiControllerItemLifeTime::Lifetime(x);
                    }
                    TsumugiControllerItemLifeTime::Lifecycle(mut x) => {
                        x = x.saturating_sub(1);
                        parcel.parcellifetime = TsumugiControllerItemLifeTime::Lifecycle(x);
                    }
                    _ => {}
                }
            }
            tsumugi_hash.1.pickup_list.retain(|val|
                match (val.parcellifetime, val.current_state) {
                    (TsumugiControllerItemLifeTime::Flash, _) => false,
                    (TsumugiControllerItemLifeTime::Once, TsumugiControllerItemState::Fulfilled) => false,
                    (TsumugiControllerItemLifeTime::Lifecycle(0), _) => false,
                    (TsumugiControllerItemLifeTime::LifeCount(0), _) => false,
                    (TsumugiControllerItemLifeTime::Lifetime(Duration::ZERO), _) => false,
                    (_, _) => true
                });
        }
        controll_loop_kit.inst_time = Instant::now();
    }

    fn transfer(&self, parcel: &mut TsumugiParcelDistributor, antenna: &mut TsumugiAntenna) {
        if let TsumugiControllerItemLifeTime::LifeCount(0) = antenna.antennalifetime {
            antenna.current_state = TsumugiControllerItemState::Fulfilled;
            return;
        }
        if let TsumugiControllerItemLifeTime::LifeCount(0) = parcel.parcellifetime {
            parcel.current_state = TsumugiControllerItemState::Fulfilled;
            return;
        }
        let antenna_state = antenna.parcel.input_item(&mut parcel.parcel, self);
        //これはOnceの場合、一回Fulfilledしたのに後続のpercelを棄却した際、Denyになってしまうのを防ぐため。
        if (antenna.antennalifetime, antenna.current_state) != (TsumugiControllerItemLifeTime::Once, TsumugiControllerItemState::Fulfilled) {
            antenna.current_state = antenna_state;
        }
        if antenna_state != TsumugiControllerItemState::Deny {
            if let TsumugiControllerItemLifeTime::LifeCount(mut x) = antenna.antennalifetime {
                x = x.saturating_sub(1);
                antenna.antennalifetime = TsumugiControllerItemLifeTime::LifeCount(x);
            }
            if let TsumugiControllerItemLifeTime::LifeCount(mut x) = parcel.parcellifetime {
                x = x.saturating_sub(1);
                parcel.parcellifetime = TsumugiControllerItemLifeTime::LifeCount(x);
            }
            if parcel.parcellifetime == TsumugiControllerItemLifeTime::Once {
                parcel.current_state = TsumugiControllerItemState::Fulfilled;
            }
        }
    }

    //注意：antenna_chain_actionはparcel_actionの後に実行すること。
    pub(crate) fn antenna_chain_action(&self, chain_item: TsumugiAntennaType, depot_hashmap_typeof: &mut DepotHashList, antenna_chain_hashlist: &mut TsumugiAntennaChainHashList) {
        match chain_item {
            TsumugiAntennaType::TsumugiAntenna(mut antenna) => {
                let tsumugi_hash_typesep = depot_hashmap_typeof.antenna_hashmap.entry(antenna.parceltype).or_insert(TsumugiParcelHashList {
                    pickup_list_withid: Default::default(),
                    recept_list_withid: Default::default(),
                    pickup_list: vec![],
                    recept_list: vec![],
                });
                if let Some(parcel_name) = &antenna.parcel_name {
                    let recept_list = tsumugi_hash_typesep.recept_list_withid.entry(parcel_name.clone()).or_insert_with(Vec::new);
                    match antenna.antenna_application {
                        TsumugiControllerApplication::New => {}
                        _ => {
                            let mut existence = false;
                            recept_list.retain(|val| {
                                let is_equal = val.antennalifetime.compare(&antenna.antennalifetime);
                                existence = is_equal || existence;
                                !is_equal
                            });
                            match antenna.antenna_application {
                                TsumugiControllerApplication::RenewOrReject => {
                                    if !existence {
                                        return;
                                    }
                                }
                                TsumugiControllerApplication::Drop => { return; }
                                _ => {}
                            }
                        }
                    }
                    if let Some(pickup_list_withid) = tsumugi_hash_typesep.pickup_list_withid.get_mut(&parcel_name.clone()) {
                        for parcel in pickup_list_withid {
                            self.transfer(parcel, &mut antenna);
                        }
                    }
                    match (antenna.antennalifetime, antenna.current_state) {
                        (TsumugiControllerItemLifeTime::Flash, _) => {}
                        (TsumugiControllerItemLifeTime::Once, TsumugiControllerItemState::Fulfilled) => {}
                        (TsumugiControllerItemLifeTime::LifeCount(0), _) => {}
                        (_, _) => { recept_list.push(antenna); }
                    }
                } else {
                    match antenna.antenna_application {
                        TsumugiControllerApplication::New => {}
                        _ => {
                            let mut existence = false;
                            tsumugi_hash_typesep.recept_list.retain(|val| {
                                let is_equal = val.antennalifetime.compare(&antenna.antennalifetime);
                                existence = is_equal || existence;
                                !is_equal
                            });
                            match antenna.antenna_application {
                                TsumugiControllerApplication::RenewOrReject => {
                                    if !existence {
                                        return;
                                    }
                                }
                                TsumugiControllerApplication::Drop => { return; }
                                _ => {}
                            }
                        }
                    }

                    for mut parcel in &mut tsumugi_hash_typesep.pickup_list {
                        self.transfer(parcel, &mut antenna);
                    }
                    match (antenna.antennalifetime, antenna.current_state) {
                        (TsumugiControllerItemLifeTime::Flash, _) => {}
                        (TsumugiControllerItemLifeTime::Once, TsumugiControllerItemState::Fulfilled) => {}
                        (TsumugiControllerItemLifeTime::LifeCount(0), _) => {}
                        (_, _) => { tsumugi_hash_typesep.recept_list.push(antenna); }
                    }
                }
            }
            TsumugiAntennaType::TsumugiAntennaChain(mut antenna_chain) => {
                match antenna_chain.chain_type {
                    TsumugiAntennaChainType::And => {
                        let antenna_list = std::mem::take(&mut antenna_chain.tsumugi_antenna_list);
                        for antenna_item in antenna_list {
                            self.antenna_chain_action(antenna_item, depot_hashmap_typeof, antenna_chain_hashlist);
                        }
                    }
                    TsumugiAntennaChainType::Next => {
                        self.antenna_chain_action(antenna_chain.tsumugi_antenna_list.swap_remove(0), depot_hashmap_typeof, antenna_chain_hashlist);
                    }
                }
                antenna_chain_hashlist.receipt_list.push(antenna_chain);
            }
            TsumugiAntennaType::TsumugiSingal(mut sig) => {
                let signal_hashlist = depot_hashmap_typeof.signal_hashmap.entry(sig.signal_name.clone()).or_insert(TsumugiSignalHashList { pickup_list: vec![], recept_list: vec![] });
                signal_hashlist.recept_list.push(sig);
            }
        }
    }

    pub(crate) fn parcel_action(&self, mut pickup_item: TsumugiParcelDistributor, depot_hashmap_typeof: &mut DepotHashList) {
        let tsumugi_parcel_hash_list = TsumugiParcelHashList {
            pickup_list_withid: Default::default(),
            recept_list_withid: Default::default(),
            pickup_list: vec![],
            recept_list: vec![],
        };
        let tsumugi_hash_typesep = depot_hashmap_typeof.antenna_hashmap.entry(pickup_item.parceltype).or_insert(tsumugi_parcel_hash_list);
        if let Some(parcel_name) = &pickup_item.parcel_name {
            let pickup_list_withid = tsumugi_hash_typesep.pickup_list_withid.entry(parcel_name.clone()).or_insert_with(Vec::new);

            match pickup_item.parcel_application {
                TsumugiControllerApplication::New => {}
                _ => {
                    let mut existence = false;
                    pickup_list_withid.retain(|val| {
                        let is_equal = val.parcellifetime.compare(&pickup_item.parcellifetime);
                        existence = is_equal || existence;
                        !is_equal
                    });
                    match pickup_item.parcel_application {
                        TsumugiControllerApplication::RenewOrReject => {
                            if !existence {
                                return;
                            }
                        }
                        TsumugiControllerApplication::Drop => { return; }
                        _ => {}
                    }
                }
            }
            if let Some(recept_list_withid) = tsumugi_hash_typesep.recept_list_withid.get_mut(&parcel_name.clone()) {
                for antenna in recept_list_withid {
                    self.transfer(&mut pickup_item, antenna);
                    if pickup_item.parcellifetime == TsumugiControllerItemLifeTime::LifeCount(0) {
                        return;
                    }
                }
            }
            pickup_list_withid.push(pickup_item);
        } else {
            //todo:ここ二度手間だからなんとか共通化したいところ
            match pickup_item.parcel_application {
                TsumugiControllerApplication::New => {}
                _ => {
                    let mut existence = false;
                    tsumugi_hash_typesep.pickup_list.retain(|val| {
                        let is_equal = val.parcellifetime.compare(&pickup_item.parcellifetime);
                        existence = is_equal || existence;
                        !is_equal
                    });
                    match pickup_item.parcel_application {
                        TsumugiControllerApplication::RenewOrReject => {
                            if !existence {
                                return;
                            }
                        }
                        TsumugiControllerApplication::Drop => { return; }
                        _ => {}
                    }
                }
            }

            for mut antenna in &mut tsumugi_hash_typesep.recept_list {
                self.transfer(&mut pickup_item, antenna);
                if pickup_item.parcellifetime == TsumugiControllerItemLifeTime::LifeCount(0) {
                    return;
                }
            }
            tsumugi_hash_typesep.pickup_list.push(pickup_item);
        }
    }

    fn antenna_chain_loop(&self, controll_loop_kit: &mut ControllLoopKitStruct) {
        let antenna_chain_hash_list = &mut controll_loop_kit.antennachain_hashmap;
        let mut next_antenna_list = Vec::new();
        for antenna_chain in &mut antenna_chain_hash_list.receipt_list {
            match antenna_chain.chain_type {
                TsumugiAntennaChainType::And => {
                    let antenna_state = antenna_chain.antenna_chain.execute_subscribe();
                    antenna_chain.current_state = antenna_state;
                    match (antenna_chain.antenna_chain_lifetime, antenna_state) {
                        (TsumugiControllerItemLifeTime::LifeCount(mut x), TsumugiControllerItemState::Fulfilled) => {
                            x = x.saturating_sub(1);
                            antenna_chain.antenna_chain_lifetime = TsumugiControllerItemLifeTime::LifeCount(x);
                        }
                        (TsumugiControllerItemLifeTime::Lifecycle(mut x), _) => {
                            x = x.saturating_sub(1);
                            antenna_chain.antenna_chain_lifetime = TsumugiControllerItemLifeTime::Lifecycle(x);
                        }
                        (TsumugiControllerItemLifeTime::Lifetime(mut x), _) => {
                            x = x.saturating_sub(controll_loop_kit.inst_time.elapsed());
                            antenna_chain.antenna_chain_lifetime = TsumugiControllerItemLifeTime::Lifetime(x);
                        }
                        (_, _) => {}
                    }
                }
                TsumugiAntennaChainType::Next => {
                    let antenna_state = antenna_chain.antenna_chain.execute_subscribe();
                    if antenna_state == TsumugiControllerItemState::Fulfilled {
                        let antenna_list = std::mem::take(&mut antenna_chain.tsumugi_antenna_list);
                        next_antenna_list.push(antenna_list);
                    }
                }
            }
        }
        for antenna_list in next_antenna_list {
            for antenna_item in antenna_list {
                self.antenna_chain_action(antenna_item, &mut controll_loop_kit.depot_hashmap_typeof, antenna_chain_hash_list);
            }
        }
        antenna_chain_hash_list.receipt_list.retain(|antenna_chain| {
            match (antenna_chain.antenna_chain_lifetime, antenna_chain.current_state) {
                (TsumugiControllerItemLifeTime::Flash, _) => false,
                (TsumugiControllerItemLifeTime::Once, TsumugiControllerItemState::Fulfilled) => false,
                (TsumugiControllerItemLifeTime::Lifetime(Duration::ZERO), _) => false,
                (TsumugiControllerItemLifeTime::LifeCount(0), _) => false,
                (TsumugiControllerItemLifeTime::Lifecycle(0), _) => false,
                (_, TsumugiControllerItemState::Eliminate) => false,
                (_, _) => true
            }
        })
    }

    fn signal_transfor(&self, pickupsig: &mut TsumugiSignal, receptsig: &mut TsumugiSignal) {
        if let TsumugiControllerItemLifeTime::LifeCount(0) = receptsig.signallifetime {
            receptsig.current_state = TsumugiControllerItemState::Fulfilled;
            return;
        }
        if let TsumugiControllerItemLifeTime::LifeCount(0) = pickupsig.signallifetime {
            pickupsig.current_state = TsumugiControllerItemState::Fulfilled;
            return;
        }
        let singalState: TsumugiControllerItemState = match &receptsig.on_receive_signal {
            None => { TsumugiControllerItemState::Fulfilled }
            Some(val) => { val.as_ref()(self) }
        };
        if (receptsig.signallifetime, receptsig.current_state) != (TsumugiControllerItemLifeTime::Once, TsumugiControllerItemState::Fulfilled) {
            receptsig.current_state = singalState;
        };
        if (pickupsig.signallifetime, pickupsig.current_state) != (TsumugiControllerItemLifeTime::Once, TsumugiControllerItemState::Fulfilled) {
            pickupsig.current_state = singalState;
        };
        if singalState != TsumugiControllerItemState::Deny {
            if let TsumugiControllerItemLifeTime::LifeCount(mut x) = receptsig.signallifetime {
                x = x.saturating_sub(1);
                receptsig.signallifetime = TsumugiControllerItemLifeTime::LifeCount(x);
            }
            if let TsumugiControllerItemLifeTime::LifeCount(mut x) = pickupsig.signallifetime {
                x = x.saturating_sub(1);
                pickupsig.signallifetime = TsumugiControllerItemLifeTime::LifeCount(x);
            }
        }
    }

    fn signal_loop(&self, controll_loop_kit: &mut ControllLoopKitStruct) {
        for sinal_hashmap in controll_loop_kit.depot_hashmap_typeof.signal_hashmap.values_mut() {
            for pickupsig in &mut sinal_hashmap.pickup_list {
                for receptsig in &mut sinal_hashmap.recept_list {
                    self.signal_transfor(pickupsig, receptsig);
                }
                match pickupsig.signallifetime {
                    TsumugiControllerItemLifeTime::Lifecycle(mut x) => {
                        x = x.saturating_sub(1);
                        pickupsig.signallifetime = TsumugiControllerItemLifeTime::Lifecycle(x);
                    }
                    TsumugiControllerItemLifeTime::Lifetime(mut x) => {
                        x = x.saturating_sub(controll_loop_kit.inst_time.elapsed());
                        pickupsig.signallifetime = TsumugiControllerItemLifeTime::Lifetime(x);
                    }
                    _ => {}
                }
            }
            for receptsig in &mut sinal_hashmap.recept_list {
                match receptsig.signallifetime {
                    TsumugiControllerItemLifeTime::Lifecycle(mut x) => {
                        x = x.saturating_sub(1);
                        receptsig.signallifetime = TsumugiControllerItemLifeTime::Lifecycle(x);
                    }
                    TsumugiControllerItemLifeTime::Lifetime(mut x) => {
                        x = x.saturating_sub(controll_loop_kit.inst_time.elapsed());
                        receptsig.signallifetime = TsumugiControllerItemLifeTime::Lifetime(x);
                    }
                    _ => {}
                }
            }
            sinal_hashmap.pickup_list.retain(|signal| {
                match (signal.signallifetime, signal.current_state) {
                    (TsumugiControllerItemLifeTime::Flash, _) => false,
                    (TsumugiControllerItemLifeTime::Once, TsumugiControllerItemState::Fulfilled) => false,
                    (TsumugiControllerItemLifeTime::Lifetime(Duration::ZERO), _) => false,
                    (TsumugiControllerItemLifeTime::LifeCount(0), _) => false,
                    (TsumugiControllerItemLifeTime::Lifecycle(0), _) => false,
                    (_, TsumugiControllerItemState::Eliminate) => false,
                    (_, _) => true
                }
            });
            sinal_hashmap.recept_list.retain(|signal| {
                match (signal.signallifetime, signal.current_state) {
                    (TsumugiControllerItemLifeTime::Flash, _) => false,
                    (TsumugiControllerItemLifeTime::Once, TsumugiControllerItemState::Fulfilled) => false,
                    (TsumugiControllerItemLifeTime::Lifetime(Duration::ZERO), _) => false,
                    (TsumugiControllerItemLifeTime::LifeCount(0), _) => false,
                    (TsumugiControllerItemLifeTime::Lifecycle(0), _) => false,
                    (_, TsumugiControllerItemState::Eliminate) => false,
                    (_, _) => true
                }
            });
        }
    }
}
#[derive(Clone)]
pub struct Debugpair{
    pub pickup_list: Vec<(TypeId,String)>,
    pub recept_list: Vec<(TypeId,String)>,
}
#[derive(Clone)]
pub struct named_Debugpair {
    pub pickup_list: Vec<(TypeId,String,String)>,
    pub recept_list: Vec<(TypeId,String,String)>,
}
#[derive(Clone)]
pub struct signal_Debugpair {
    pub pickup_list: Vec<(String,String)>,
    pub recept_list: Vec<(String,String)>,
}
#[derive(Clone)]
pub struct antennachain_Debugpair {
    pub list: Vec<String>,
}
#[derive(Clone)]
pub struct DebugKit{
    pub plane_name:String,
    pub parcel:Debugpair,
    pub named_parcel: named_Debugpair,
    pub signal:signal_Debugpair,
    pub antennachain:antennachain_Debugpair
}
impl ControllLoopKitStruct {
    fn debug(&self,plane_name:String)->DebugKit{
        let parcel:(Vec<_>,Vec<_>) = self.depot_hashmap_typeof.antenna_hashmap.iter().map(|v|{
            let pickup = v.1.pickup_list.iter().map(|w|{
                (v.0.clone(),w.distributor_name.clone().unwrap_or("No Name".to_string()))
            }).collect::<Vec<_>>();
            let recept = v.1.recept_list.iter().map(|w|{
                (v.0.clone(),w.antenna_name.clone().unwrap_or("No Name".to_string()))
            }).collect::<Vec<_>>();
            (pickup,recept)
        }).unzip();
        let named_parcel:(Vec<_>,Vec<_>) = self.depot_hashmap_typeof.antenna_hashmap.iter().map(|v|{
            let pickup = v.1.pickup_list_withid.iter().map(|w|{
                w.1.iter().map(|x|{
                    (v.0.clone(),w.0.clone(),x.distributor_name.clone().unwrap_or("No Name".to_string()))
                }).collect::<Vec<_>>()
            }).collect::<Vec<_>>();
            let recept = v.1.recept_list_withid.iter().map(|w|{
                w.1.iter().map(|x|{
                    (v.0.clone(),w.0.clone(),x.antenna_name.clone().unwrap_or("No Name".to_string()))
                }).collect::<Vec<_>>()
            }).collect::<Vec<_>>();
            (pickup,recept)
        }).unzip();
        let signal:(Vec<_>,Vec<_>) = self.depot_hashmap_typeof.signal_hashmap.iter().map(|v|{
            let pickup = v.1.pickup_list.iter().map(|w|{
                (v.0.clone(),w.signal_name.clone())
            }).collect::<Vec<_>>();
            let recept = v.1.recept_list.iter().map(|w|{
                (v.0.clone(),w.signal_name.clone())
            }).collect::<Vec<_>>();
            (pickup,recept)
        }).unzip();
        let antenna_chain = self.antennachain_hashmap.receipt_list.iter().map(|v|{
            v.chain_name.clone().unwrap_or("No Name".to_string())
        }).collect::<Vec<_>>();
        DebugKit{
            plane_name,
            parcel: Debugpair { pickup_list: parcel.0.into_iter().flatten().collect::<Vec<_>>(), recept_list: parcel.1.into_iter().flatten().collect::<Vec<_>>() },
            named_parcel: named_Debugpair { pickup_list: named_parcel.0.into_iter().flatten().flatten().collect::<Vec<_>>(), recept_list: named_parcel.1.into_iter().flatten().flatten().collect::<Vec<_>>() },
            signal: signal_Debugpair { pickup_list: signal.0.into_iter().flatten().collect::<Vec<_>>(), recept_list: signal.1.into_iter().flatten().collect::<Vec<_>>() },
            antennachain: antennachain_Debugpair { list: antenna_chain }
        }
    }
}