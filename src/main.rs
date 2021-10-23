use std::future::Future;
use std::sync::{Arc, Mutex, mpsc};
use std::pin::Pin;
use std::task::{Context};
use std::sync::mpsc::{Sender, Receiver};
use std::any::{Any, TypeId};
use std::path::Path;
use tsumugi::antenna::{TsumugiAntenna};
use tsumugi::distributor::TsumugiParcelDistributor;
use tsumugi::controller::{TsumugiChannelSenders, TsumugiController, TsumugiObject, TsumugiControllerTrait, TsumugiControllerItemState, TsumugiControllerItemLifeTime, TsumugiController_thread};
use tsumugi::parcel_receptor::TsumugiParcelReceptor;
use tsumugiKeyboardInput::spown_windows_key_controller;
use tsumugi::signal::TsumugiSignal;
use tsumugi::antenna_chain::TsumugiAntennaType;
use tsumugiWindowController::spown_window_handler;
use tsumuGraphic_DirectX12::spown_direct_x12_handler;
use tsumuObject::{spown_3d_object_handler, Tsumugi3DObject};
use tsumuStockCPU::spown_object_stock_handler;

struct ObjectA {
    input_item: Arc<Mutex<i32>>,
    input_item_local: Arc<Mutex<i32>>,
    local_tsumugi_sender: TsumugiChannelSenders,
    object:Tsumugi3DObject,
}

impl ObjectA {
    fn spowntsumugiantenna(&self, tc: &TsumugiController) -> TsumugiAntenna {
        let itemlock = self.input_item.clone();
        let tsumugi_pr = TsumugiParcelReceptor::new(Parcel { package: 0 }).subscribe(
            Arc::new(move |parcel| {
                let mut item = itemlock.lock().unwrap();
                *item += parcel.parcel.package;
                dbg!(*item);
                TsumugiControllerItemState::Deny
            }));
        let tsumugi_antenna = tsumugi_pr.into();
        tsumugi_antenna
    }
    fn spownresetantenna(&self, tc: &TsumugiController) -> TsumugiAntenna {
        let itemlock = self.input_item.clone();
        let tsumugi_pr = TsumugiParcelReceptor {
            parcel: Box::new(Reset { package: 0 }),
            subscribe: Some(Arc::new(move |reset, tct| {
                let mut item = itemlock.lock().unwrap();
                *item = reset.parcel.package;
                dbg!(*item);
                TsumugiControllerItemState::Deny
            })),
        };
        tsumugi_pr.into()
    }
    fn spownkeyantenna(&self, tc: &TsumugiController) -> TsumugiAntennaType {
        let itemlock = self.input_item.clone();
        let tsumugi_pr = TsumugiSignal::new("w")
            .lifetime(TsumugiControllerItemLifeTime::Eternal)
            .subscribe(Arc::new(move || {
                let mut item = itemlock.lock().unwrap();
                *item = 500;
                TsumugiControllerItemState::Fulfilled
            }));
        tsumugi_pr.into()
    }
}

struct Observer {
    input_item: Arc<Mutex<i32>>,
    local_tsumugi_sender: TsumugiChannelSenders,
}

//todo:Observerの仕様どうするか検討
impl Observer {
    fn new(item: i32, tc: TsumugiChannelSenders) -> Observer {
        Observer { input_item: Arc::new(Mutex::new(item)), local_tsumugi_sender: tc.clone() }
    }
    fn use_state(&self, number: i32) {
        if let Ok(mut inputitem) = self.input_item.lock() {
            *inputitem = number;
        }
    }
}

impl TsumugiObject for ObjectA {
    fn on_create(&self, tc: &TsumugiController_thread) {
        let mut receive_ticket = self.spowntsumugiantenna(&tc.tc);
        tc.tc.global_channel_sender.recept_channel_sender.send(receive_ticket.into());
        let mut recet_ticket = self.spownresetantenna(&tc.tc);
        tc.tc.global_channel_sender.recept_channel_sender.send(recet_ticket.into());
        let mut key_ticket = self.spownkeyantenna(&tc.tc);
        tc.tc.global_channel_sender.recept_channel_sender.send(key_ticket);
        dbg!(tc.tc.global_connect_tsumugi_controller.lock().unwrap().keys());
        self.object.create3d_object(&tc.tc);
    }
}

pub fn spown_object_controller(tc: &Box<TsumugiController>) -> Box<TsumugiController> {
    let mut newtc = tc.spown("tsumugiobject".to_string());

    newtc.set_objects(vec![
        Box::new(ObjectA { input_item: Arc::new(Mutex::new(200)), input_item_local: Arc::new(Mutex::new(0)), local_tsumugi_sender: newtc.local_channel_sender.clone(), object: Tsumugi3DObject::new("shapell",Path::new("Asset/shapell_Mtoon.vrm")) }),
        // Box::new(ObjectA { input_item: Arc::new(Mutex::new(500)), input_item_local: Arc::new(Mutex::new(0)), local_tsumugi_sender: newtc.local_channel_sender.clone() })
    ]);
    return newtc;
}

#[derive(Clone)]
struct Parcel {
    package: i32,
}

#[derive(Clone)]
struct Reset {
    package: i32,
}

fn main() {
    let mut tsumugiroot = TsumugiController::new("Tsumugi".to_string());
    //todo:spown_object_stock_handlerが初期化できてないとTsumugiStockが見つからずにエラーが出る可能性がある。遅延実行や先行処理をできるようにしたい
    tsumugiroot.execute_tsumugi_functions(vec![spown_3d_object_handler,spown_object_controller, spown_window_handler,spown_object_stock_handler,spown_direct_x12_handler]);
    tsumugiroot.global_channel_sender.pickup_channel_sender.send(TsumugiParcelDistributor::new(Parcel { package: 12 }).into());
    dbg!(tsumugiroot.global_connect_tsumugi_controller.lock().unwrap().keys());
    loop {
        let mut word = String::new();
        std::io::stdin().read_line(&mut word).ok();
        let answer = word.trim().to_string();
        if answer == "end" {
            break;
        } else if answer == "r" {
            tsumugiroot.local_channel_sender.pickup_channel_sender.send(TsumugiParcelDistributor::new(Reset { package: 0 }).into());
        } else {
            tsumugiroot.local_channel_sender.pickup_channel_sender.send(TsumugiParcelDistributor::new(Parcel { package: 13 }).into());
            tsumugiroot.global_connect_tsumugi_controller.lock().unwrap().get("tsumugiobject").unwrap().local_channel_sender.pickup_channel_sender.send(TsumugiParcelDistributor::new(Parcel { package: 14 }).into());
            tsumugiroot.global_connect_tsumugi_controller.lock().unwrap().get("tsumugiobject").unwrap().global_channel_sender.pickup_channel_sender.send(TsumugiParcelDistributor::new(Parcel { package: 15 }).into());
        }
    }
    println!("Hello, world!");
}
