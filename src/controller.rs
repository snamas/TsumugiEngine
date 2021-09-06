use std::any::{TypeId};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{mpsc, Arc, Mutex, Condvar};
use std::thread;
use std::thread::JoinHandle;
use std::collections::HashMap;
use crate::antenna::{TsumugiAntenna, TsumugiFuture, TsumugiCurrentState};
use crate::distributor::{TsumugiParcelDistributor, ParcelLifeTime};
use tsumugi_macro::{TsumugiAnyTrait};
use crate::antenna_chain::TsumugiAntennaChain;

pub struct TsumugiController {
    pub local_channel_sender:TsumugiChannelSenders,
    pub global_channel_sender:TsumugiChannelSenders,
    connect_tsumugi_controller: Vec<String>,
    pub global_connect_tsumugi_controller: Arc<Mutex<HashMap<String,Box<TsumugiController>>>>,
    pub tsumugi_controller_name: String,
    tsumugi_object_vector: Vec<Box<dyn TsumugiObject + Send>>
}

#[derive(Clone)]
pub struct TsumugiChannelSenders{
    pub pickup_channel_sender: Sender<TsumugiParcelDistributor>,
    pub recept_channel_sender: Sender<TsumugiAntenna>,
    pub recept_chain_channel_sender: Sender<TsumugiAntennaChain>,

}
pub struct TsumugiParcelHashList{
    pickup_list_withid:HashMap<u64, TsumugiParcelDistributor>,
    receipt_list_withid:HashMap<u64,TsumugiAntenna>,
    pickup_list: Vec<TsumugiParcelDistributor>,
    receipt_list:Vec<TsumugiAntenna>
}

pub trait TsumugiObject {
    fn on_create(&self,tc:&TsumugiController);
}
pub trait TsumugiControllerTrait {
    fn new(tsumuginame: String) -> Box<TsumugiController>;
    fn spown(self:&Box<Self>, tsumuginame: String) -> Box<TsumugiController>;
    fn set_object(&mut self, tsumugi_object_list: Vec<Box<dyn TsumugiObject + Send>>);
    fn execute_tsumugi_functions(self:&Box<Self>, tsumugi_functions:Vec<Box<dyn Fn(&Box<TsumugiController>) -> Box<TsumugiController>>>);
    fn execute_tsumugi_thread(&self, receipt_channnel_receiver: Receiver<TsumugiAntenna>, receipt_chain_channnel_receiver: Receiver<TsumugiAntennaChain>,pickup_channnel_receiver: Receiver<TsumugiParcelDistributor>) -> JoinHandle<()>;
}

impl TsumugiControllerTrait for TsumugiController {
    fn new(tsumuginame: String) -> Box<TsumugiController> {
        let (recept_channel_sender, receipt_channnel_receiver): (Sender<TsumugiAntenna>, Receiver<TsumugiAntenna>) = mpsc::channel();
        let (recept_chain_channel_sender, receipt_chain_channnel_receiver): (Sender<TsumugiAntennaChain>, Receiver<TsumugiAntennaChain>) = mpsc::channel();
        let (pickup_channel_sender, pickup_channnel_receiver): (Sender<TsumugiParcelDistributor>, Receiver<TsumugiParcelDistributor>) = mpsc::channel();
        let tsumugi_channel_senders = TsumugiChannelSenders{pickup_channel_sender,recept_channel_sender,recept_chain_channel_sender};
        let mut tsumugi_connect_list: Vec<String> = Vec::new();
        let mut tsumugi_object_list: Vec<Box<dyn TsumugiObject + Send>> = Vec::new();
        let mut tc = Box::new(TsumugiController {
            local_channel_sender: tsumugi_channel_senders.clone(),
            global_channel_sender: tsumugi_channel_senders,
            connect_tsumugi_controller: tsumugi_connect_list,
            global_connect_tsumugi_controller: Arc::new(Mutex::new(HashMap::new())),
            tsumugi_controller_name: tsumuginame,
            tsumugi_object_vector: tsumugi_object_list
        });
        tc.execute_tsumugi_thread(receipt_channnel_receiver, receipt_chain_channnel_receiver,pickup_channnel_receiver);
        return tc;
    }

    fn spown(self:&Box<Self>, tsumuginame: String) -> Box<TsumugiController> {
        let mut tc = Self::new(tsumuginame);
        tc.global_channel_sender = self.global_channel_sender.clone();
        return tc;
    }

    fn set_object(&mut self, mut tsumugi_object_list: Vec<Box<dyn TsumugiObject + Send>>) {
        for tsumugi_object in &tsumugi_object_list {
            tsumugi_object.on_create(self);
        }
        self.tsumugi_object_vector.append(&mut tsumugi_object_list);
    }
    /// TsumugiController生成関数の配列を受け取ってそれを使ってTsumugiControllerを生成していくよ
    fn execute_tsumugi_functions(self:&Box<Self>, create_tsumugi_controller_funclist: Vec<Box<dyn Fn(&Box<TsumugiController>) -> Box<TsumugiController>>>) {
        for tsumugi_function in create_tsumugi_controller_funclist {
            let mut tc_new = tsumugi_function(self);
            self.global_connect_tsumugi_controller.lock().unwrap().insert(tc_new.tsumugi_controller_name.clone(), tc_new as Box<TsumugiController>);
        }
    }
    fn execute_tsumugi_thread(&self, receipt_channnel_receiver: Receiver<TsumugiAntenna>, receipt_chain_channnel_receiver: Receiver<TsumugiAntennaChain>, pickup_channnel_receiver: Receiver<TsumugiParcelDistributor>) -> JoinHandle<()> {

        thread::spawn(move || {
            //todo:うまいことロックを使いこなそうcondvarというやつをつかって
            //todo:あとhashを値の管理に使う。
            let mut tumugi_receipt_list: Vec<Box<dyn TsumugiFuture + Send>> = Vec::new();
            let mut pickup_list: Vec<Box<dyn TsumugiAnyTrait + Send>> = Vec::new();
            let condvar = Condvar::new();
            let mutex = Mutex::new(());
            let lock = mutex.lock().unwrap();
            //condvar.wait(lock).unwrap();
            let mut tsumugi_hashmap_typeof:HashMap<TypeId,TsumugiParcelHashList> = HashMap::new();
            loop {
                let mut receipt_iter = receipt_channnel_receiver.try_iter();
                for receive_item in receipt_iter {
                    let tsumugi_parcel_hash_list = TsumugiParcelHashList{
                        pickup_list_withid: Default::default(),
                        receipt_list_withid: Default::default(),
                        pickup_list:vec![],
                        receipt_list: vec![]
                    };
                    dbg!(receive_item.parceltype);
                    let tsumugi_hash_typesep = tsumugi_hashmap_typeof.entry(receive_item.parceltype).or_insert(tsumugi_parcel_hash_list);
                    tsumugi_hash_typesep.receipt_list.push(receive_item);
                }
                let mut pickup_iter  = pickup_channnel_receiver.try_iter();
                for pickup_item in pickup_iter {
                    let tsumugi_parcel_hash_list = TsumugiParcelHashList{
                        pickup_list_withid: Default::default(),
                        receipt_list_withid: Default::default(),
                        pickup_list:vec![],
                        receipt_list: vec![]
                    };
                    dbg!(pickup_item.parceltype);
                    let tsumugi_hash_typesep = tsumugi_hashmap_typeof.entry(pickup_item.parceltype).or_insert(tsumugi_parcel_hash_list);
                    tsumugi_hash_typesep.pickup_list.push(pickup_item);
                }
                let mut i = 0;
                for  tsumugi_hash in tsumugi_hashmap_typeof.iter_mut(){
                    for pickupitem in tsumugi_hash.1.pickup_list.iter_mut(){
                        for receiptitem in tsumugi_hash.1.receipt_list.iter_mut(){
                            receiptitem.current_state = receiptitem.parcel.input_item(&mut pickupitem.parcel);
                        }
                    }
                    //todo:ここpickupitemの性質（変更だけ受け取りたい場合もあるよ
                    tsumugi_hash.1.receipt_list.retain(|x| x.current_state != TsumugiCurrentState::Fulfilled);
                    tsumugi_hash.1.pickup_list.retain(|x| x.parcellifetime != ParcelLifeTime::Flash);
                }
            }
        }
        )
    }
}