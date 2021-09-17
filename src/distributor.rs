use std::any::{TypeId, Any};
use crate::controller::{TsumugiControllerApplication, TsumugiControllerItemLifeTime, TsumugiControllerItemState};

pub struct TsumugiParcelDistributor {
    pub parcel:Box<dyn Any + Send>,
    pub parceltype: TypeId,
    pub parcellifetime: TsumugiControllerItemLifeTime,
    pub parcel_name: Option<String>,
    pub parcel_application:TsumugiControllerApplication,
    pub current_state:TsumugiControllerItemState
}

impl TsumugiParcelDistributor{
    pub fn new<T: 'static + Send>(tsumugi_parcel:T) ->Self{
        TsumugiParcelDistributor{
            parcel: Box::new(tsumugi_parcel),
            parceltype: TypeId::of::<T>(),
            parcellifetime: TsumugiControllerItemLifeTime::Once,
            parcel_name: None,
            parcel_application: TsumugiControllerApplication::New,
            current_state: TsumugiControllerItemState::Untreated
        }
    }
    pub fn name(mut self,name:impl ToString)->Self{
        self.parcel_name = Some(name.to_string());
        self
    }
    pub fn lifetime(mut self, lifetime: TsumugiControllerItemLifeTime) ->Self{
        self.parcellifetime = lifetime;
        self
    }
    pub fn application(mut self,parcel_application:TsumugiControllerApplication)->Self{
        self.parcel_application = parcel_application;
        self
    }
}