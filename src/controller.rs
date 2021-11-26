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

pub struct TsumugiController_thread {
    pub tc: TsumugiController,
    tsumugi_object_vector: Vec<Box<dyn TsumugiObject + Send>>,
}

#[derive(Clone)]
pub struct TsumugiController {
    pub local_channel_sender: TsumugiChannelSenders,
    pub global_channel_sender: TsumugiChannelSenders,
    pub(crate) connect_tsumugi_controller: Vec<String>,
    pub global_connect_tsumugi_controller: Arc<Mutex<HashMap<String, Box<TsumugiController>>>>,
    pub tsumugi_controller_name: String,
    pub(crate) tsumugi_object_sender: Sender<Box<dyn TsumugiObject + Send>>,
}

pub struct Thread_receivers {
    distributer: Receiver<TsumugiDistributor>,
    antenna: Receiver<TsumugiAntennaType>,
    object: Receiver<Box<dyn TsumugiObject + Send>>,
}

#[derive(Clone)]
pub struct TsumugiChannelSenders {
    pub pickup_channel_sender: Sender<TsumugiDistributor>,
    pub recept_channel_sender: Sender<TsumugiAntennaType>,
}

pub struct TsumugiParcelHashList {
    pickup_list_withid: HashMap<String, Vec<TsumugiParcelDistributor>>,
    recept_list_withid: HashMap<String, Vec<TsumugiAntenna>>,
    pickup_list: Vec<TsumugiParcelDistributor>,
    recept_list: Vec<TsumugiAntenna>,
}

pub struct TsumugiSignalHashList {
    pickup_list: Vec<TsumugiSignal>,
    recept_list: Vec<TsumugiSignal>,
}

struct DepotHashList {
    antenna_hashmap: HashMap<TypeId, TsumugiParcelHashList>,
    signal_hashmap: HashMap<String, TsumugiSignalHashList>,
}

pub struct TsumugiAntennaChainHashList {
    receipt_list: Vec<TsumugiAntennaChain>,
}

struct ControllLoopKitStruct {
    thread_receivers: Thread_receivers,
    inst_time: Instant,
    depot_hashmap_typeof: DepotHashList,
    antennachain_hashmap: TsumugiAntennaChainHashList,
    object_list: Vec<Box<dyn TsumugiObject + Send>>,
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
    fn on_create(&self, tc: &TsumugiController_thread);
}

pub trait TsumugiControllerTrait {
    fn new(tsumuginame: String) -> Box<TsumugiController>;
    fn spown(self: &Box<Self>, tsumuginame: String) -> Box<TsumugiController>;
    fn set_object(&mut self, tsumugi_object:Box<dyn TsumugiObject + Send>);
    fn set_objects(&mut self, tsumugi_object_list: Vec<Box<dyn TsumugiObject + Send>>);
    fn find(&self, Controller_name: &str) -> Option<TsumugiChannelSenders>;
    fn execute_tsumugi_functions(self: &Box<Self>, tsumugi_functions: Vec<fn(&Box<TsumugiController>) -> Box<TsumugiController>>);
    fn execute_tsumugi_thread(&self, thread_receivers: Thread_receivers) -> JoinHandle<()>;
}

impl TsumugiControllerTrait for TsumugiController {
    //todo:box化する意味はあんまりない気がする。
    //todo:newとspownでコードの一部がかぶっているのをなんとかしたい
    fn new(tsumuginame: String) -> Box<TsumugiController> {
        let (recept_channel_sender, receipt_channnel_receiver) = mpsc::channel();
        let (pickup_channel_sender, pickup_channnel_receiver) = mpsc::channel();
        let (object_sender, object_receiver) = mpsc::channel();
        let tsumugi_channel_senders = TsumugiChannelSenders { pickup_channel_sender, recept_channel_sender };
        let mut tsumugi_connect_list: Vec<String> = Vec::new();
        let mut tc = Box::new(TsumugiController {
            local_channel_sender: tsumugi_channel_senders.clone(),
            global_channel_sender: tsumugi_channel_senders,
            connect_tsumugi_controller: tsumugi_connect_list,
            global_connect_tsumugi_controller: Arc::new(Mutex::new(HashMap::new())),
            tsumugi_controller_name: tsumuginame,
            tsumugi_object_sender: object_sender,
        });
        let receivers = Thread_receivers {
            distributer: pickup_channnel_receiver,
            antenna: receipt_channnel_receiver,
            object: object_receiver,
        };
        tc.execute_tsumugi_thread(receivers);
        return tc;
    }

    fn spown(self: &Box<Self>, tsumuginame: String) -> Box<TsumugiController> {
        let (recept_channel_sender, receipt_channnel_receiver) = mpsc::channel();
        let (pickup_channel_sender, pickup_channnel_receiver) = mpsc::channel();
        let (object_sender, object_receiver) = mpsc::channel();
        let tsumugi_channel_senders = TsumugiChannelSenders { pickup_channel_sender, recept_channel_sender };
        let mut tsumugi_connect_list: Vec<String> = Vec::new();
        let mut tc = Box::new(TsumugiController {
            local_channel_sender: tsumugi_channel_senders.clone(),
            global_channel_sender: self.global_channel_sender.clone(),
            connect_tsumugi_controller: tsumugi_connect_list,
            global_connect_tsumugi_controller: self.global_connect_tsumugi_controller.clone(),
            tsumugi_controller_name: tsumuginame,
            tsumugi_object_sender: object_sender,
        });
        let receivers = Thread_receivers {
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
    fn set_objects(&mut self, mut tsumugi_object_list: Vec<Box<dyn TsumugiObject + Send>>) {
        for tsumugi_object in tsumugi_object_list {
            self.tsumugi_object_sender.send(tsumugi_object);
        }
    }
    fn find(&self, Controller_name: &str) -> Option<TsumugiChannelSenders> {
        Some(self.global_connect_tsumugi_controller.lock().unwrap().get(Controller_name)?.local_channel_sender.clone())
    }
    /// TsumugiController生成関数の配列を受け取って,それを使ってTsumugiControllerを生成していくよ
    fn execute_tsumugi_functions(self: &Box<Self>, create_tsumugi_controller_funclist: Vec<fn(&Box<TsumugiController>) -> Box<TsumugiController>>) {
        for tsumugi_function in create_tsumugi_controller_funclist {
            let mut tc_new = tsumugi_function(self);
            self.global_connect_tsumugi_controller.lock().unwrap().insert(tc_new.tsumugi_controller_name.clone(), tc_new as Box<TsumugiController>);
        }
    }
    fn execute_tsumugi_thread(&self, thread_receivers: Thread_receivers) -> JoinHandle<()> {
        let mut tc_thread = TsumugiController_thread { tc: self.clone(), tsumugi_object_vector: vec![] };
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
                if let Some(debug_plane) = tc_thread.tc.find("TsumugiDebugWin"){
                    let debugkit = controll_loop_kit.debug(plane_name.clone());
                    debug_plane.pickup_channel_sender.send(TsumugiParcelDistributor::new(debugkit).lifetime(Once).displayname(plane_name.clone()).into());
                }
            }
        }
        )
    }
}

impl TsumugiController_thread {
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
    fn thread_loop_antenna_parcel(&mut self, controll_loop_kit: &mut ControllLoopKitStruct) {
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
    fn antenna_chain_action(&self, chain_item: TsumugiAntennaType, depot_hashmap_typeof: &mut DepotHashList, antenna_chain_hashlist: &mut TsumugiAntennaChainHashList) {
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

    fn parcel_action(&self, mut pickup_item: TsumugiParcelDistributor, depot_hashmap_typeof: &mut DepotHashList) {
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
#[cfg(test)]
mod tests {
    use std::any::{Any, TypeId};
    use crate::parcel_receptor_with_channel::TsumugiParcelReceptorWithChannel;
    use crate::antenna_chain::{TsumugiSpownReceiver, TsumugiAntennaType};
    use crate::antenna_chain::TsumugiReceptorChain;
    use std::collections::{HashMap, HashSet};
    use crate::controller::{DepotHashList, TsumugiAntennaChainHashList, TsumugiControllerApplication, TsumugiController, TsumugiChannelSenders, TsumugiObject, TsumugiParcelHashList, TsumugiControllerItemLifeTime, TsumugiControllerItemState, ControllLoopKitStruct, Thread_receivers, TsumugiController_thread};
    use crate::distributor::{TsumugiParcelDistributor, TsumugiDistributor};
    use std::sync::mpsc::{Receiver, Sender};
    use std::time::{Instant};
    use std::sync::{mpsc, Mutex, Arc};
    use crate::parcel_receptor::TsumugiParcelReceptor;
    use crate::antenna::TsumugiAntenna;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[derive(Clone)]
    struct Parcel {
        package: i32,
    }

    #[derive(Clone)]
    struct ParcelStr {
        package: String,
    }

    #[derive(Clone)]
    struct Backet {
        package: i32,
    }

    impl ControllLoopKitStruct {
        fn new() -> (Self, TsumugiChannelSenders) {
            let (recept_channel_sender, receipt_channnel_receiver) = mpsc::channel();
            let (pickup_channel_sender, pickup_channnel_receiver) = mpsc::channel();
            let (object_sender, object_receiver) = mpsc::channel();
            let tsumugi_channel_senders = TsumugiChannelSenders { pickup_channel_sender, recept_channel_sender };
            let thread_receivers = Thread_receivers {
                distributer: pickup_channnel_receiver,
                antenna: receipt_channnel_receiver,
                object: object_receiver,
            };
            let controll_test_kit_struct = ControllLoopKitStruct {
                thread_receivers: thread_receivers,
                inst_time: Instant::now(),
                depot_hashmap_typeof: DepotHashList { antenna_hashmap: HashMap::new(), signal_hashmap: HashMap::new() },
                antennachain_hashmap: TsumugiAntennaChainHashList { receipt_list: Vec::new() },
                object_list: vec![],
            };
            (controll_test_kit_struct, tsumugi_channel_senders)
        }
        fn checkhashmap(&mut self, typeid: TypeId) -> &mut TsumugiParcelHashList {
            self.depot_hashmap_typeof.antenna_hashmap.get_mut(&typeid).unwrap()
        }
    }

    impl TsumugiController_thread {
        pub(crate) fn new(tcs: TsumugiChannelSenders) -> Self {
            let (object_sender, object_receiver) = mpsc::channel();
            TsumugiController_thread {
                tc: TsumugiController {
                    local_channel_sender: tcs.clone(),
                    global_channel_sender: tcs,
                    connect_tsumugi_controller: vec![],
                    global_connect_tsumugi_controller: Arc::new(Mutex::new(Default::default())),
                    tsumugi_controller_name: "".to_string(),
                    tsumugi_object_sender: object_sender,
                },
                tsumugi_object_vector: vec![],
            }
        }
        fn execute_tsumugi_nothread(&mut self, controll_loop_kit: &mut ControllLoopKitStruct) {
            self.thread_loop_antenna_parcel(controll_loop_kit);
        }
    }

    impl TsumugiParcelHashList {
        ///(pickup_list, pickup_list_withid, recept_list. recept_list_withid)の順番で出力する。
        fn hashListCount(&self) -> (usize, usize, usize, usize) {
            (self.pickup_list.len(), self.pickup_list_withid.len(), self.recept_list.len(), self.recept_list_withid.len())
        }
    }

    #[test]
    fn chain_parse_test() {
        let (mut controll_loop_kit, tsumugi_channel_senders) = ControllLoopKitStruct::new();
        let mut tc = TsumugiController_thread::new(tsumugi_channel_senders.clone());
        let mut tsumugi_pr = TsumugiParcelReceptorWithChannel::<Parcel>::new().antenna_name("pa");
        let mut tb_pr = TsumugiParcelReceptorWithChannel::<Backet>::new();
        let mut chain = crate::antenna_chain!(tsumugi_pr.clone(),tb_pr);
        chain.chain_name = Some("chain".into());
        let chain2 = crate::antenna_chain!(tsumugi_pr,chain);
        let mut antenna_hashmap_typeof = DepotHashList { antenna_hashmap: HashMap::new(), signal_hashmap: HashMap::new() };
        let mut antennachain_hashmap = TsumugiAntennaChainHashList { receipt_list: Vec::new() };
        //[antenna[parcel(pa),antenna[paecel(pa),Backet2(None)]]
        tc.antenna_chain_action(chain2.into(), &mut antenna_hashmap_typeof, &mut antennachain_hashmap);
        {
            let antenna_name = antenna_hashmap_typeof.antenna_hashmap.iter().map(|(typeid, antenna)| {
                antenna.recept_list.iter().map(|x| { x.antenna_name.clone() }).collect::<Vec<Option<String>>>()
            }).collect::<HashSet<Vec<Option<String>>>>();
            let mut check_antennavec = HashSet::new();
            check_antennavec.insert(vec![Some("pa".into()), Some("pa".into())]);
            check_antennavec.insert(vec![None]);
            assert_eq!(antenna_name, check_antennavec);
        }
        {
            let antenna_name = antennachain_hashmap.receipt_list.iter().map(|x| {
                x.chain_name.clone()
            }).collect::<Vec<Option<String>>>();
            let check_antennachainvec = vec![Some("chain".into()), None];
            assert_eq!(antenna_name, check_antennachainvec);
        }
    }

    #[test]
    fn parcelparse_test() {
        let (mut controll_loop_kit, tsumugi_channel_senders) = ControllLoopKitStruct::new();
        let mut tc = TsumugiController_thread::new(tsumugi_channel_senders.clone());
        let parcel = TsumugiParcelDistributor::new(Parcel { package: 1 });
        let mut parcel_name = TsumugiParcelDistributor::new(Parcel { package: 1 });
        parcel_name.parcel_name = Some("parcel_is_here".into());
        let mut backet_name = TsumugiParcelDistributor::new(Backet { package: 1 });
        backet_name.parcel_name = Some("backet_is_here".into());
        let parcelpack = vec![parcel, parcel_name, backet_name];
        let mut depot_hashmap_typeof = DepotHashList { antenna_hashmap: HashMap::new(), signal_hashmap: HashMap::new() };
        for parcel in parcelpack {
            tc.parcel_action(parcel, &mut depot_hashmap_typeof);
        }
        {
            let parcel_noname = depot_hashmap_typeof.antenna_hashmap.iter().map(|(_typeid, value)| {
                value.pickup_list.iter().map(|x| {
                    x.parceltype
                }).collect::<Vec<TypeId>>()
            }).collect::<HashSet<Vec<TypeId>>>();
            let mut parcel_typelist = HashSet::new();
            parcel_typelist.insert(vec![TypeId::of::<Parcel>()]);
            parcel_typelist.insert(vec![]);
            assert_eq!(parcel_noname, parcel_typelist);
        }
        {
            let parcel_name = depot_hashmap_typeof.antenna_hashmap.iter().map(|(_typeid, value)| {
                value.pickup_list_withid.iter().map(|(name, _parcel)| { name.clone() }).collect::<Vec<String>>()
            }).collect::<HashSet<Vec<String>>>();
            let mut parcel_typelist = HashSet::new();
            parcel_typelist.insert(vec!["parcel_is_here".into()]);
            parcel_typelist.insert(vec!["backet_is_here".into()]);
            assert_eq!(parcel_name, parcel_typelist);
        }
    }

    #[test]
    fn controller_nothread_test() {
        let (mut controll_loop_kit, tsumugi_channel_senders) = ControllLoopKitStruct::new();
        let mut tc = TsumugiController_thread::new(tsumugi_channel_senders.clone());
        {
            //parcelを送る（Once）
            let new_parcel = TsumugiParcelDistributor::new(Parcel { package: 10 });
            tsumugi_channel_senders.pickup_channel_sender.send(new_parcel.into());
            tc.execute_tsumugi_nothread(&mut controll_loop_kit);
            let hashmap = controll_loop_kit.checkhashmap(TypeId::of::<Parcel>());
            assert_eq!(hashmap.hashListCount(), (1, 0, 0, 0));
            assert_eq!(hashmap.pickup_list.get(0).unwrap().parcellifetime, TsumugiControllerItemLifeTime::Once);
        }
        let mut parcelpackage = Arc::new(Mutex::new(1));
        let mut parcelpack_clone = parcelpackage.clone();
        let tsumugi_pr = TsumugiParcelReceptor::new(Parcel { package: 0 }).subscribe(
            Arc::new(move |parcel| {
                *parcelpack_clone.lock().unwrap() += parcel.parcel.package;
                TsumugiControllerItemState::Fulfilled
            }));
        {
            //Antennaを送る（Eternal,反応）
            tsumugi_channel_senders.recept_channel_sender.send(tsumugi_pr.clone().into());
            tc.execute_tsumugi_nothread(&mut controll_loop_kit);
            let hashmap = controll_loop_kit.checkhashmap(TypeId::of::<Parcel>());
            assert_eq!(*parcelpackage.lock().unwrap(), 11);
            assert_eq!(hashmap.hashListCount(), (0, 0, 1, 0));
        }
        {
            //Antennaを送る（Flash,無反応）
            let mut tsumugi_antenna: TsumugiAntenna = tsumugi_pr.clone().into();
            tsumugi_antenna.antennalifetime = TsumugiControllerItemLifeTime::Flash;
            tsumugi_channel_senders.recept_channel_sender.send(tsumugi_antenna.into());
            tc.execute_tsumugi_nothread(&mut controll_loop_kit);
            let hashmap = controll_loop_kit.checkhashmap(TypeId::of::<Parcel>());
            assert_eq!(*parcelpackage.lock().unwrap(), 11);
            assert_eq!(hashmap.hashListCount(), (0, 0, 1, 0));
        }
        {
            //Antennaを送る（Once,無反応）
            let mut tsumugi_antenna: TsumugiAntenna = tsumugi_pr.clone().into();
            tsumugi_antenna.antennalifetime = TsumugiControllerItemLifeTime::Once;
            tsumugi_channel_senders.recept_channel_sender.send(tsumugi_antenna.into());
            tc.execute_tsumugi_nothread(&mut controll_loop_kit);
            let hashmap = controll_loop_kit.checkhashmap(TypeId::of::<Parcel>());
            assert_eq!(*parcelpackage.lock().unwrap(), 11);
            assert_eq!(hashmap.hashListCount(), (0, 0, 2, 0));
        }
        {
            //parcelを送る（Once）
            let new_parcel = TsumugiParcelDistributor::new(Parcel { package: 20 });
            tsumugi_channel_senders.pickup_channel_sender.send(new_parcel.into());
            tc.execute_tsumugi_nothread(&mut controll_loop_kit);
            let hashmap = controll_loop_kit.checkhashmap(TypeId::of::<Parcel>());
            assert_eq!(*parcelpackage.lock().unwrap(), 51);
            assert_eq!(hashmap.hashListCount(), (0, 0, 1, 0));
        }
        //parcelを0にリセット
        *parcelpackage.lock().unwrap() = 0;
        //コントローラーをリセット
        let (mut controll_loop_kit, tsumugi_channel_senders) = ControllLoopKitStruct::new();
        {
            //parcelを送る（LifeCount(2)）
            let mut new_parcel = TsumugiParcelDistributor::new(Parcel { package: 10 });
            new_parcel.parcellifetime = TsumugiControllerItemLifeTime::LifeCount(2);
            tsumugi_channel_senders.pickup_channel_sender.send(new_parcel.into());
            tc.execute_tsumugi_nothread(&mut controll_loop_kit);
            let hashmap = controll_loop_kit.checkhashmap(TypeId::of::<Parcel>());
            assert_eq!(hashmap.hashListCount(), (1, 0, 0, 0));
            assert_eq!(hashmap.pickup_list.get(0).unwrap().parcellifetime, TsumugiControllerItemLifeTime::LifeCount(2));
        }
        {
            //Antennaを送る（Flash,反応）
            let mut tsumugi_antenna: TsumugiAntenna = tsumugi_pr.clone().into();
            tsumugi_antenna.antennalifetime = TsumugiControllerItemLifeTime::Flash;
            tsumugi_channel_senders.recept_channel_sender.send(tsumugi_antenna.into());
            tc.execute_tsumugi_nothread(&mut controll_loop_kit);
            let hashmap = controll_loop_kit.checkhashmap(TypeId::of::<Parcel>());
            assert_eq!(*parcelpackage.lock().unwrap(), 10);
            assert_eq!(hashmap.hashListCount(), (1, 0, 0, 0));
            assert_eq!(hashmap.pickup_list.get(0).unwrap().parcellifetime, TsumugiControllerItemLifeTime::LifeCount(1));
        }
        {
            //Antennaを送る（LifeCount(2),反応,Lifecycle(2),反応）ParcelはⅠ回目でLifeCountを消費して終了
            let mut tsumugi_antenna: TsumugiAntenna = tsumugi_pr.clone().into();
            tsumugi_antenna.antennalifetime = TsumugiControllerItemLifeTime::LifeCount(2);
            tsumugi_channel_senders.recept_channel_sender.send(tsumugi_antenna.into());
            let mut tsumugi_antenna: TsumugiAntenna = tsumugi_pr.clone().into();
            tsumugi_antenna.antennalifetime = TsumugiControllerItemLifeTime::Lifecycle(2);
            tsumugi_channel_senders.recept_channel_sender.send(tsumugi_antenna.into());
            tc.execute_tsumugi_nothread(&mut controll_loop_kit);
            let hashmap = controll_loop_kit.checkhashmap(TypeId::of::<Parcel>());
            assert_eq!(*parcelpackage.lock().unwrap(), 20);
            assert_eq!(hashmap.hashListCount(), (0, 0, 2, 0));
            assert_eq!(hashmap.recept_list.get(0).unwrap().antennalifetime, TsumugiControllerItemLifeTime::LifeCount(1));
            assert_eq!(hashmap.recept_list.get(1).unwrap().antennalifetime, TsumugiControllerItemLifeTime::Lifecycle(1));
        }
        {
            //Antennaを送る（renew）
            let mut tsumugi_antenna: TsumugiAntenna = tsumugi_pr.clone().into();
            tsumugi_antenna.antennalifetime = TsumugiControllerItemLifeTime::LifeCount(3);
            tsumugi_antenna.antenna_application = TsumugiControllerApplication::Renew;
            tsumugi_channel_senders.recept_channel_sender.send(tsumugi_antenna.into());
            tc.execute_tsumugi_nothread(&mut controll_loop_kit);
            let hashmap = controll_loop_kit.checkhashmap(TypeId::of::<Parcel>());
            assert_eq!(*parcelpackage.lock().unwrap(), 20);
            assert_eq!(hashmap.hashListCount(), (0, 0, 1, 0));
            assert_eq!(hashmap.recept_list.get(0).unwrap().antennalifetime, TsumugiControllerItemLifeTime::LifeCount(3));
        }
    }

    #[test]
    fn controller_antennachain_nothread_test() {
        let (mut controll_loop_kit, tsumugi_channel_senders) = ControllLoopKitStruct::new();
        let mut tc = TsumugiController_thread::new(tsumugi_channel_senders.clone());
        let mut parcelpackage = Arc::new(Mutex::new("NoMessage".to_string()));
        {
            //AntennaChainをつくる。AntennaChain[ParcelStr("pr"),AntennaChain[tsumugi_pr,tb_pr]]
            let mut tsumugi_pr = TsumugiParcelReceptorWithChannel::<ParcelStr>::new();
            let mut tb_pr = TsumugiParcelReceptorWithChannel::<Backet>::new();
            let mut chain = crate::antenna_chain!(tsumugi_pr.clone(),tb_pr);
            let mut parcelpack_clone = parcelpackage.clone();
            let mut chain = chain.set_name("chain").subscribe(Box::new(move |(parcel, backet), send| {
                let mut p = parcel.try_iter();
                let mut b = backet.try_iter();
                if let (Some(pitem), Some(bitem)) = (p.next(), b.next()) {
                    *parcelpack_clone.lock().unwrap() = pitem.package;
                    send.clone().unwrap().send(());
                    return TsumugiControllerItemState::Fulfilled;
                }
                return TsumugiControllerItemState::Deny;
            }));
            let mut parcel_receptname = tsumugi_pr.recept_name("pr");
            let mut parcelpack_clone = parcelpackage.clone();
            let mut chain2bool = AtomicBool::new(false);
            let chain2 = crate::antenna_chain!(parcel_receptname,chain).subscribe(Box::new(move |(parcel, antenna_chain_recv), send| {
                let mut p = parcel.try_iter();
                let mut ac = antenna_chain_recv.try_iter();
                if let Some(acitem) = ac.next() {
                    chain2bool.store(true, Ordering::SeqCst);
                }
                if let (Some(pitem), true) = (p.next(), chain2bool.load(Ordering::SeqCst)) {
                    *parcelpack_clone.lock().unwrap() = pitem.package;
                    return TsumugiControllerItemState::Fulfilled;
                }
                return TsumugiControllerItemState::Deny;
            }));
            tsumugi_channel_senders.recept_channel_sender.send(chain2.into());
            tc.execute_tsumugi_nothread(&mut controll_loop_kit);
            let hashmap = controll_loop_kit.checkhashmap(TypeId::of::<ParcelStr>());
            assert_eq!(hashmap.hashListCount(), (0, 0, 1, 1));
            dbg!(&controll_loop_kit.depot_hashmap_typeof.antenna_hashmap.get_mut(&TypeId::of::<ParcelStr>()).unwrap().recept_list_withid.get_mut(&"pr".to_string()).unwrap().get(0).unwrap().parcel_name.as_ref().unwrap());
            let hashmap = controll_loop_kit.checkhashmap(TypeId::of::<Backet>());
            assert_eq!(hashmap.hashListCount(), (0, 0, 1, 0));
        }
        {
            //parcelを送る（Once）
            let mut new_parcel = TsumugiParcelDistributor::new(ParcelStr { package: "ParcelIsReceived".to_string() });
            tsumugi_channel_senders.pickup_channel_sender.send(new_parcel.into());
            tc.execute_tsumugi_nothread(&mut controll_loop_kit);
            let hashmap = controll_loop_kit.checkhashmap(TypeId::of::<ParcelStr>());
            assert_eq!(hashmap.hashListCount(), (0, 0, 1, 1));
            assert_eq!(*parcelpackage.lock().unwrap(), "NoMessage".to_string());
        }
        {
            //parcelとbacketを送る（Once）
            let mut new_parcel = TsumugiParcelDistributor::new(ParcelStr { package: "ParcelIsReceived".to_string() });
            tsumugi_channel_senders.pickup_channel_sender.send(new_parcel.into());
            let mut new_backet = TsumugiParcelDistributor::new(Backet { package: 51 });
            tsumugi_channel_senders.pickup_channel_sender.send(new_backet.into());
            tc.execute_tsumugi_nothread(&mut controll_loop_kit);
            let hashmap = controll_loop_kit.checkhashmap(TypeId::of::<ParcelStr>());
            assert_eq!(hashmap.hashListCount(), (0, 0, 1, 1));
            assert_eq!(*parcelpackage.lock().unwrap(), "ParcelIsReceived".to_string());
        }
        {
            //parcel("pr")を送る（Once）
            let mut new_parcel = TsumugiParcelDistributor::new(ParcelStr { package: "NamedParcelIsReceived".to_string() }).parcelname("pr");
            tsumugi_channel_senders.pickup_channel_sender.send(new_parcel.into());
            tc.execute_tsumugi_nothread(&mut controll_loop_kit);
            let hashmap = controll_loop_kit.checkhashmap(TypeId::of::<ParcelStr>());
            assert_eq!(*parcelpackage.lock().unwrap(), "NamedParcelIsReceived".to_string());
            assert_eq!(hashmap.hashListCount(), (0, 1, 1, 1));
        }
    }
}