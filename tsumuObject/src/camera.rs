use nalgebra::{Point, Point2, Point3};
use tsumugi::controller::{TsumugiControllerItemLifeTime, TsumugiObject, TsumugiPortal, TsumugiPortalPlaneLocal};
use tsumugi::distributor::{TsumugiDistributor, TsumugiParcelDistributor};
#[derive(Clone)]
pub struct Camera {
    position: nalgebra::Point3<f32>,
    rotate: nalgebra::Point2<f32>,
    size: nalgebra::Point3<f32>,
    pub name: &'static str,
}

impl Camera {
    pub fn new()->Self{
        Camera{
            position: Point3::new(0.0, 0.0, 0.0),
            rotate: Point2::new(0.0, 0.0),
            size: Point3::new(0.0, 0.0, 0.0),
            name: "Camera"
        }
    }
    pub fn name(mut self,name:&'static str)->Self{
        self.name = name;
        self
    }

    pub fn update(&self,tc: &TsumugiPortal){
        tc.global_channel_sender.pickup_channel_sender.send(TsumugiParcelDistributor::new(self.clone()).displayname("camera").lifetime(TsumugiControllerItemLifeTime::Once).into());
    }
}
impl TsumugiObject for Camera{
    fn on_create(&self, tc: &TsumugiPortalPlaneLocal) {
        self.update(&tc.tp);
    }
}