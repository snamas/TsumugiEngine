mod shapelloader;
mod boxloader;
mod test_shader_PS;
mod test_shader_VS;
mod shapell_shader_PS;
mod shapell_shader_VS;
mod camera_trait;

use std::future::Future;
use std::sync::{Arc, Mutex, mpsc};
use std::pin::Pin;
use std::task::{Context};
use std::sync::mpsc::{Sender, Receiver};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::path::Path;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use nalgebra::{Translation3, UnitQuaternion, Vector3};
use tsumuDebugwin::spown_debug_window_handler;
use tsumugi::antenna::{TsumugiAntenna};
use tsumugi::distributor::TsumugiParcelDistributor;
use tsumugi::controller::{TsumugiChannelSenders, TsumugiPortal, TsumugiObject, TsumugiControllerTrait, TsumugiControllerItemState, TsumugiControllerItemLifeTime, TsumugiPortalPlaneLocal};
use tsumugi::parcel_receptor::TsumugiParcelReceptor;
use tsumugiKeyboardInput::spown_windows_key_controller;
use tsumugi::signal::TsumugiSignal;
use tsumugi::antenna_chain::TsumugiAntennaType;
use tsumugiWindowController::spown_window_handler;
use tsumuGraphic_DirectX12::spown_direct_x12_handler;
use tsumuObject::{spown_3d_object_handler, Tsumugi3DObject};
use tsumuFigureStockCPU::{Attribute, ObjectLoader, spown_figure_stock_handler, Texcoord, TsumugiVertexBinary};
use tsumugiShaderStock::{ConstantBuffer, spown_shader_stock_handler, TsumugiMaterial};
use tsumuObject::camera::Camera;
use crate::boxloader::SampleBox;
use crate::camera_trait::object_camera;
use crate::shapelloader::Shapell;

struct scene_kari {
    local_tsumugi_sender: TsumugiChannelSenders,
    shapell_object:Tsumugi3DObject,
    shapell:Shapell,
    sample_box_object:Tsumugi3DObject,
    sample_box:SampleBox,
    camera:Arc<Mutex<Camera>>
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

impl TsumugiObject for scene_kari {
    fn on_create(&self, tc: &TsumugiPortalPlaneLocal) {
        sleep(Duration::new(0,100));
        dbg!(tc.tp.global_connect_tsumugi_controller.lock().unwrap().keys());
        self.sample_box_object.create3d_object(&tc.tp);
        self.shapell_object.create3d_object(&tc.tp);
        self.shapell.hair.store_material(&tc.tp);
        self.shapell.blazer.store_material(&tc.tp);
        self.shapell.body.store_material(&tc.tp);
        self.shapell.clothes.store_material(&tc.tp);
        self.sample_box.material.store_material(&tc.tp);
        self.camera.create_object_camera(&tc.tp);
        let mut shapell_mat = self.shapell.hair.clone();
        let tp = tc.tp.clone();
        thread::spawn(move ||{
            let mut num = 0f32;
            loop {
                sleep(Duration::new(0,100));
                let bufarr = unsafe{
                    std::mem::transmute::<_, [u8;4]>(num.cos()) };
                shapell_mat.material.buffer = vec![bufarr.to_vec()];
                shapell_mat.store_material(&tp);
                num += 0.01;
            }
        });
    }
}

pub fn spown_object_controller(tc: &Box<TsumugiPortal>) -> Box<TsumugiPortal> {
    let mut newtc = tc.spown("tsumugiscene".to_string());
    newtc.set_objects(vec![
        Box::new(scene_kari {
            local_tsumugi_sender: newtc.local_channel_sender.clone(),
            shapell_object: Tsumugi3DObject::new("shapell", Path::new("Asset/shapell_Mtoon.vrm"),"ShapellMaterial", Shapell::load),
            shapell: Shapell::default(),
            sample_box_object: Tsumugi3DObject::new("samplebox", Path::new("Asset/Box.glb"),"SampleBoxMaterial", SampleBox::load),
            sample_box: SampleBox::default(),
            camera: Arc::new(Mutex::new(Camera::new(Translation3::new(0.2f32,0f32,-3f32), UnitQuaternion::from_scaled_axis(Vector3::new(0f32, std::f32::consts::PI, 0f32)))))
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
    let mut tsumugiroot = TsumugiPortal::new("Tsumugi".to_string());
    //todo:spown_object_stock_handlerが初期化できてないとTsumugiStockが見つからずにエラーが出る可能性がある。遅延実行や先行処理をできるようにしたい
    tsumugiroot.execute_tsumugi_functions(vec![spown_3d_object_handler, spown_object_controller, spown_window_handler,spown_shader_stock_handler ,spown_figure_stock_handler, spown_direct_x12_handler, spown_debug_window_handler]);
    dbg!(tsumugiroot.global_connect_tsumugi_controller.lock().unwrap().keys());
    loop {
    }
    println!("Hello, world!");
}
