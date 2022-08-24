use std::sync::{Arc, Mutex};
use nalgebra::{Isometry3, Vector3};
use tsumugi::controller::{TsumugiControllerItemState, TsumugiControllerTrait, TsumugiPortal, TsumugiPortalPlaneLocal};
use tsumugi::parcelreceptor_novalue::TsumugiParcelReceptorNoVal;
use tsumuObject::camera::Camera;

pub trait object_camera{
    fn create_object_camera(&self, tc: &TsumugiPortal);
}

impl object_camera for Arc<Mutex<Camera>>{
    fn create_object_camera(&self, tc: &TsumugiPortal) {
        let thread_camera = self.clone();
        let func = move |arc_hwnd: &TsumugiParcelReceptorNoVal<i16>, tpl: &TsumugiPortalPlaneLocal| {
            let thread_handleWindow = arc_hwnd.parcel.clone().unwrap();
            if let Ok(mut camera) = thread_camera.lock(){
                camera.position.vector = &camera.position.vector + camera.rotation.transform_vector(&Vector3::new(0f32,0f32,*thread_handleWindow as f32 * 0.01f32));
                camera.update(&tpl.tp);
            }
            TsumugiControllerItemState::Fulfilled
        };
        let recept_object = TsumugiParcelReceptorNoVal::new().subscribe_with_portal(Arc::new(func)).to_antenna().parcelname("mouse_wheel").displayname("camera_recept");
        tc.global_channel_sender.wait(recept_object.into());
    }
}