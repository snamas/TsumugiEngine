use std::any::{TypeId, Any};
use std::sync::mpsc::{Sender, Receiver, Iter};
use std::sync::{mpsc, Arc, Mutex};
use std::task::Poll;
use std::thread;
use std::thread::JoinHandle;
use std::pin::Pin;
use std::marker::PhantomPinned;
use std::collections::HashMap;

pub trait TsumugiFuture: TsumugiTypeChacher {
    fn poll(self: &mut Self) -> Poll<()>;
    fn input_item(self: &mut Self, input_item: Box<dyn TsumugiTypeChacher + Send>);
}

pub trait TsumugiTypeChacher {
    fn as_any(&mut self) -> &mut dyn Any;
    fn typehash(&self) -> TypeId;
}

pub trait TsumugiObject {
    fn on_create(&self,tc:&TsumugiController);
}

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
    pub pickup_channel_sender: Sender<Box<dyn TsumugiTypeChacher + Send>>,
    pub receipt_channel_sender: Sender<Box<dyn TsumugiFuture + Send >>
}
pub trait TsumugiControllerTrait {
    fn new(tsumuginame: String) -> Box<TsumugiController>;
    fn spown(self:&Box<Self>, tsumuginame: String) -> Box<TsumugiController>;
    fn set_object(&mut self, tsumugi_object_list: Vec<Box<dyn TsumugiObject + Send>>);
    fn execute_tsumugi_functions(self:&Box<Self>, tsumugi_functions:Vec<Box<dyn Fn(&Box<TsumugiController>) -> Box<TsumugiController>>>);
    fn execute_tsumugi_thread(&self, receipt_channnel_receiver: Receiver<Box<dyn TsumugiFuture + Send>>, pickup_channnel_receiver: Receiver<Box<dyn TsumugiTypeChacher + Send>>) -> JoinHandle<()>;
}

impl TsumugiControllerTrait for TsumugiController {
    fn new(tsumuginame: String) -> Box<TsumugiController> {
        let (receipt_channel_sender, receipt_channnel_receiver): (Sender<Box<dyn TsumugiFuture + Send>>, Receiver<Box<dyn TsumugiFuture + Send>>) = mpsc::channel();
        let (pickup_channel_sender, pickup_channnel_receiver): (Sender<Box<dyn TsumugiTypeChacher + Send>>, Receiver<Box<dyn TsumugiTypeChacher + Send>>) = mpsc::channel();
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

    fn execute_tsumugi_functions(self:&Box<Self>, create_tsumugi_controller_funclist: Vec<Box<dyn Fn(&Box<TsumugiController>) -> Box<TsumugiController>>>) {
        for tsumugi_function in create_tsumugi_controller_funclist {
            let mut tc_new = tsumugi_function(self);
            self.global_connect_tsumugi_controller.lock().unwrap().insert(tc_new.tsumugi_controller_name.clone(), tc_new as Box<TsumugiController>);
        }
    }
    fn execute_tsumugi_thread(&self, receipt_channnel_receiver: Receiver<Box<dyn TsumugiFuture + Send>>, pickup_channnel_receiver: Receiver<Box<dyn TsumugiTypeChacher + Send>>) -> JoinHandle<()> {

        thread::spawn(move || {
            let mut receiveList: Iter<Box<dyn TsumugiFuture + Send>> = receipt_channnel_receiver.iter();
            let mut tumugi_receipt_list: Vec<Box<dyn TsumugiFuture + Send>> = Vec::new();
            let mut pickup_list: Vec<Box<dyn TsumugiTypeChacher + Send>> = Vec::new();
            if let Some(mut receiveItem) = receiveList.next() {
                loop {
                    let mut sendList: Vec<_> = pickup_channnel_receiver.try_iter().collect();
                    while let Some(sendItem) = sendList.pop() {
                        if receiveItem.typehash() == sendItem.typehash() {
                            receiveItem.input_item(sendItem);
                        }
                    }
                }
            }
        }
        )
    }
}