use std::sync::Arc;
use nalgebra::{Isometry3, IsometryMatrix3, Matrix4, MatrixN, Perspective3, Point, Point2, Point3, Projective3, Quaternion, Rotation3, Similarity3, SimilarityMatrix3, Translation3, UnitQuaternion, Vector, Vector3};
use tsumugi::controller::{TsumugiControllerApplication, TsumugiControllerItemLifeTime, TsumugiControllerItemState, TsumugiControllerTrait, TsumugiObject, TsumugiPortal, TsumugiPortalPlaneLocal};
use tsumugi::distributor::{TsumugiDistributor, TsumugiParcelDistributor};
use tsumugi::parcelreceptor_novalue::TsumugiParcelReceptorNoVal;

#[derive(Clone)]
pub struct Camera {
    pub position: nalgebra::Translation3<f32>,
    pub rotation: nalgebra::UnitQuaternion<f32>,
    aspect:f32,
    fov_y:f32,
    z_near:f32,
    z_far:f32,
    pub name: &'static str,
}

impl Default for Camera {
    fn default() -> Self {
        Camera{
            position: Translation3::identity(),
            rotation: UnitQuaternion::identity(),
            aspect: 16.0 / 9.0,
            fov_y: std::f32::consts::PI / 4.0,
            z_near: 1.0,
            z_far: 10000.0,
            name: "Camera"
        }
    }
}
impl Camera {
    pub fn new(postion:Translation3<f32>,rotation:UnitQuaternion<f32>)->Self{
        Camera{
            position: postion,
            rotation: rotation,
            aspect: 16.0 / 9.0,
            fov_y: std::f32::consts::PI / 4.0,
            z_near: 1.0,
            z_far: 10000.0,
            name: "Camera"
        }
    }
    pub fn name(mut self,name:&'static str)->Self{
        self.name = name;
        self
    }
    pub fn world_to_view(&self) -> Matrix4<f32> {
        Isometry3::from_parts(self.position,self.rotation).to_homogeneous()
    }
    pub fn view_to_projection(&self) -> Matrix4<f32> {
        Perspective3::new(self.aspect,self.fov_y,self.z_near,self.z_far).to_homogeneous()
    }
    pub fn update(&self,tc: &TsumugiPortal){
        tc.find("tsumugi3dObject").unwrap().send(TsumugiParcelDistributor::new(self.clone()).displayname("camera").application(TsumugiControllerApplication::Renew).lifetime(TsumugiControllerItemLifeTime::Once).into());
    }
    //todo:現在dynを入れないとあかんので、dynなしでの実行をしたいね
    pub fn fetch(tc:&TsumugiPortal,receive_camera_func: fn(&Camera) -> TsumugiControllerItemState){
        let func = move |arc_hwnd: &TsumugiParcelReceptorNoVal<Camera>, tpl: &TsumugiPortalPlaneLocal| {
            let thread_handleWindow = arc_hwnd.parcel.clone().unwrap();
            receive_camera_func(&*thread_handleWindow)
        };
        let recept_object = TsumugiParcelReceptorNoVal::new().subscribe_with_portal(Arc::new(func)).to_antenna().displayname("camera_object");
        tc.find("tsumugi3dObject").unwrap().wait(recept_object.into());
    }
}
impl TsumugiObject for Camera{
    fn on_create(&self, tc: &TsumugiPortalPlaneLocal) {

    }
}