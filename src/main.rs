use std::future::Future;
use std::sync::{Arc, Mutex, mpsc};
use std::pin::Pin;
use std::task::{Context};
use std::sync::mpsc::{Sender, Receiver};
use std::any::{Any, TypeId};
use tsumugiEngine::antenna::{TsumugiAntenna,  TsumugiCurrentState};
use tsumugiEngine::distributor::TsumugiParcelDistributor;
use tsumugiEngine::antenna_chain::test;
use tsumugi_macro::{TsumugiAny};
use tsumugiEngine::controller::{TsumugiChannelSenders, TsumugiController, TsumugiObject, TsumugiControllerTrait};
use tsumugiEngine::parcel_receptor::TsumugiParcelReceptor;

struct ObjectA {
    input_item: Arc<Mutex<i32>>,
    input_item_local: Arc<Mutex<i32>>,
    local_tsumugi_sender:TsumugiChannelSenders
}

impl ObjectA {
    fn spowntsumugiantenna(&self,tc:&TsumugiController) -> TsumugiAntenna {
        let itemlock = self.input_item.clone();
        let tsumugi_pr = TsumugiParcelReceptor {
            parcel: Box::new(Parcel { package: 0 }),
            on_change: Arc::new(move |parcel| {
                let mut item = itemlock.lock().unwrap();
                *item += parcel.parcel.package;
                dbg!(*item);
                TsumugiCurrentState::Untreated
            }),
        };
        let tsumugi_antenna= tsumugi_pr.into();
        tsumugi_antenna
    }
}
struct Observer{
    input_item: Arc<Mutex<i32>>,
    local_tsumugi_sender:TsumugiChannelSenders
}
impl Observer{
    fn new(item:i32,tc:TsumugiChannelSenders)->Observer{
        Observer{ input_item: Arc::new(Mutex::new(item)), local_tsumugi_sender: tc.clone() }
    }
    fn use_state(&self,number:i32){
        if let Ok(mut inputitem) = self.input_item.lock(){
            *inputitem = number;
        }
    }
}
impl TsumugiObject for ObjectA {
    fn on_create(&self, tc: &TsumugiController) {
        let mut receive_ticket = self.spowntsumugiantenna(tc);
        tc.global_channel_sender.recept_channel_sender.send(*Box::new(receive_ticket));
    }
}

pub fn spown_object_controller(tc: &Box<TsumugiController>) -> Box<TsumugiController> {
    let mut newtc = tc.spown("tsumugiobject".to_string());

    newtc.set_object(vec![
        Box::new(ObjectA { input_item: Arc::new(Mutex::new(200)), input_item_local: Arc::new(Mutex::new(0)), local_tsumugi_sender: newtc.local_channel_sender.clone() }),
        Box::new(ObjectA { input_item: Arc::new(Mutex::new(500)), input_item_local: Arc::new(Mutex::new(0)), local_tsumugi_sender: newtc.local_channel_sender.clone() })
    ]);
    return newtc;
}

#[derive(Clone,TsumugiAny)]
struct Parcel {
    package: i32,
}

#[derive(Clone,TsumugiAny)]
struct Backet2 {
    package: i32,
}

fn main() {
    let mut tsumugiroot = TsumugiController::new("Tsumugi".to_string());
    tsumugiroot.execute_tsumugi_functions(vec![Box::new(spown_object_controller)]);
    tsumugiroot.global_channel_sender.pickup_channel_sender.send(TsumugiParcelDistributor::new(Parcel { package: 12 }));
    test();
    loop {
        let mut word = String::new();
        std::io::stdin().read_line(&mut word).ok();
        let answer = word.trim().to_string();
        if answer == "end" {
            break;
        }
        tsumugiroot.local_channel_sender.pickup_channel_sender.send(TsumugiParcelDistributor::new(Parcel { package: 13 }));
        tsumugiroot.global_connect_tsumugi_controller.lock().unwrap().get("tsumugiobject").unwrap().local_channel_sender.pickup_channel_sender.send(TsumugiParcelDistributor::new(Parcel { package: 14 }));
        tsumugiroot.global_connect_tsumugi_controller.lock().unwrap().get("tsumugiobject").unwrap().global_channel_sender.pickup_channel_sender.send(TsumugiParcelDistributor::new(Parcel { package: 15 }));
        tsumugiroot.local_channel_sender.pickup_channel_sender.send(TsumugiParcelDistributor::new(Backet2 { package: 16 }));

    }
    println!("Hello, world!");
}
