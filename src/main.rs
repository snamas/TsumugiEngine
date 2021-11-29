mod shapelloader;
mod boxloader;

use std::future::Future;
use std::sync::{Arc, Mutex, mpsc};
use std::pin::Pin;
use std::task::{Context};
use std::sync::mpsc::{Sender, Receiver};
use std::any::{Any, TypeId};
use std::path::Path;
use tsumuDebugwin::spown_debug_window_handler;
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
use tsumuStockCPU::{Attribute, ObjectLoader, spown_object_stock_handler, Texcoord, TsumugiVertexBinary};
use crate::boxloader::SampleBox;
use crate::shapelloader::Shapell;

struct ObjectA {
    input_item: Arc<Mutex<i32>>,
    input_item_local: Arc<Mutex<i32>>,
    local_tsumugi_sender: TsumugiChannelSenders,
    shapell:Tsumugi3DObject,
    sample_box:Tsumugi3DObject,
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
        dbg!(tc.tc.global_connect_tsumugi_controller.lock().unwrap().keys());
        self.sample_box.create3d_object(&tc.tc);
        self.shapell.create3d_object(&tc.tc);
    }
}

pub fn spown_object_controller(tc: &Box<TsumugiController>) -> Box<TsumugiController> {
    let mut newtc = tc.spown("tsumugiobject".to_string());

    newtc.set_objects(vec![
        Box::new(ObjectA {
            input_item: Arc::new(Mutex::new(200)),
            input_item_local: Arc::new(Mutex::new(0)),
            local_tsumugi_sender: newtc.local_channel_sender.clone(),
            shapell: Tsumugi3DObject::new("shapell", Path::new("Asset/shapell_Mtoon.vrm"), Shapell::load),
            sample_box: Tsumugi3DObject::new("samplebox",Path::new("Asset/Box.glb"), SampleBox::load)
        }),
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
    tsumugiroot.execute_tsumugi_functions(vec![spown_3d_object_handler,spown_object_controller, spown_window_handler,spown_object_stock_handler,spown_direct_x12_handler,spown_debug_window_handler]);
    dbg!(tsumugiroot.global_connect_tsumugi_controller.lock().unwrap().keys());
    loop {

    }
    println!("Hello, world!");
}
