pub mod antenna;
pub mod distributor;

use std::any::{TypeId, Any};
use std::sync::mpsc::{Sender, Receiver, Iter};
use std::sync::{mpsc, Arc, Mutex, Condvar};
use std::task::Poll;
use std::thread;
use std::thread::JoinHandle;
use std::pin::Pin;
use std::marker::PhantomPinned;
use std::collections::HashMap;
use crate::antenna::{TsumugiAntenna, TsumugiFuture};
use crate::distributor::TsumugiParcelDistributor;
use tsumugi_any::{TsumugiAnyTrait};

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
    pub receipt_channel_sender: Sender<TsumugiAntenna>
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
    fn execute_tsumugi_thread(&self, receipt_channnel_receiver: Receiver<TsumugiAntenna>, pickup_channnel_receiver: Receiver<TsumugiParcelDistributor>) -> JoinHandle<()>;
}

impl TsumugiControllerTrait for TsumugiController {
    fn new(tsumuginame: String) -> Box<TsumugiController> {
        let (receipt_channel_sender, receipt_channnel_receiver): (Sender<TsumugiAntenna>, Receiver<TsumugiAntenna>) = mpsc::channel();
        let (pickup_channel_sender, pickup_channnel_receiver): (Sender<TsumugiParcelDistributor>, Receiver<TsumugiParcelDistributor>) = mpsc::channel();
        let tsumugiChannelSenders = TsumugiChannelSenders{pickup_channel_sender,receipt_channel_sender};
        let mut tsumugi_connect_list: Vec<String> = Vec::new();
        let mut tsumugi_object_list: Vec<Box<dyn TsumugiObject + Send>> = Vec::new();
        let mut tc = Box::new(TsumugiController {
            local_channel_sender: tsumugiChannelSenders.clone(),
            global_channel_sender: tsumugiChannelSenders,
            connect_tsumugi_controller: tsumugi_connect_list,
            global_connect_tsumugi_controller: Arc::new(Mutex::new(HashMap::new())),
            tsumugi_controller_name: tsumuginame,
            tsumugi_object_vector: tsumugi_object_list
        });
        tc.execute_tsumugi_thread(receipt_channnel_receiver, pickup_channnel_receiver);
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
    fn execute_tsumugi_thread(&self, receipt_channnel_receiver: Receiver<TsumugiAntenna>, pickup_channnel_receiver: Receiver<TsumugiParcelDistributor>) -> JoinHandle<()> {

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
                while let Some(receiveItem) = receipt_iter.next() {
                    let tsumugi_parcel_hash_list = TsumugiParcelHashList{
                        pickup_list_withid: Default::default(),
                        receipt_list_withid: Default::default(),
                        pickup_list:vec![],
                        receipt_list: vec![]
                    };
                    dbg!(receiveItem.parceltype);
                    let tsumugi_hash_typesep = tsumugi_hashmap_typeof.entry(receiveItem.parceltype).or_insert(tsumugi_parcel_hash_list);
                    tsumugi_hash_typesep.receipt_list.push(receiveItem);
                }
                let mut pickup_iter  = pickup_channnel_receiver.try_iter();
                while let Some(pickup_item) = pickup_iter.next() {
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
                            i = i+1;
                            dbg!(i);
                            receiptitem.parcel.input_item(&mut pickupitem.parcel);
                        }
                    }
                    //todo:ここpickupitemの性質（変更だけ受け取りたい場合もあるよ
                    tsumugi_hash.1.pickup_list.clear();
                }
            }
        }
        )
    }
}