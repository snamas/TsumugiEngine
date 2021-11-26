use std::any::{TypeId, Any};
use crate::controller::{TsumugiControllerApplication, TsumugiControllerItemLifeTime, TsumugiControllerItemState};
use crate::signal::TsumugiSignal;

pub enum TsumugiDistributor {
    TPDistributor(TsumugiParcelDistributor),
    TsumugiSignal(TsumugiSignal),
}

pub struct TsumugiParcelDistributor {
    pub parcel: Box<dyn Any + Send>,
    pub parceltype: TypeId,
    pub parcellifetime: TsumugiControllerItemLifeTime,
    pub parcel_name: Option<String>,
    pub distributor_name: Option<String>,
    pub parcel_application: TsumugiControllerApplication,
    pub current_state: TsumugiControllerItemState,
}

impl TsumugiParcelDistributor {
    pub fn new<T: 'static + Send>(tsumugi_parcel: T) -> Self {
        TsumugiParcelDistributor {
            parcel: Box::new(tsumugi_parcel),
            parceltype: TypeId::of::<T>(),
            parcellifetime: TsumugiControllerItemLifeTime::Once,
            parcel_name: None,
            distributor_name: None,
            parcel_application: TsumugiControllerApplication::New,
            current_state: TsumugiControllerItemState::Untreated,
        }
    }
    pub fn parcelname(mut self, name: impl ToString) -> Self {
        self.parcel_name = Some(name.to_string());
        self
    }
    pub fn displayname(mut self, name: impl ToString) -> Self {
        self.distributor_name = Some(name.to_string());
        self
    }
    pub fn lifetime(mut self, lifetime: TsumugiControllerItemLifeTime) -> Self {
        self.parcellifetime = lifetime;
        self
    }
    pub fn application(mut self, parcel_application: TsumugiControllerApplication) -> Self {
        self.parcel_application = parcel_application;
        self
    }
}

impl TsumugiDistributor {
    pub fn name(mut self, name: impl ToString) -> Self {
        match self {
            TsumugiDistributor::TPDistributor(mut val) => {
                val.parcel_name = Some(name.to_string());
                TsumugiDistributor::TPDistributor(val)
            }
            TsumugiDistributor::TsumugiSignal(mut val) => {
                val.signal_name = name.to_string();
                TsumugiDistributor::TsumugiSignal(val)
            }
        }
    }
    pub fn lifetime(mut self, lifetime: TsumugiControllerItemLifeTime) -> Self {
        match self {
            TsumugiDistributor::TPDistributor(mut val) => {
                val.parcellifetime = lifetime;
                TsumugiDistributor::TPDistributor(val)
            }
            TsumugiDistributor::TsumugiSignal(mut val) => {
                val.signallifetime = lifetime;
                TsumugiDistributor::TsumugiSignal(val)
            }
        }
    }
}

impl From<TsumugiParcelDistributor> for TsumugiDistributor {
    fn from(val: TsumugiParcelDistributor) -> Self {
        TsumugiDistributor::TPDistributor(val)
    }
}

impl From<TsumugiSignal> for TsumugiDistributor {
    fn from(val: TsumugiSignal) -> Self {
        TsumugiDistributor::TsumugiSignal(val)
    }
}