use std::any::{TypeId, Any};
use crate::parcel_receptor_with_channel::TsumugiParcelReceptorWithChannel;
use crate::parcel_receptor::TsumugiParcelReceptor;
use crate::parcel_receptor_return_value::TsumugiParcelReceptorReturnValue;
use crate::controller::{TsumugiControllerApplication, TsumugiControllerItemState, TsumugiControllerItemLifeTime, TsumugiPortalPlaneLocal};
use crate::parcelreceptor_novalue::TsumugiParcelReceptorNoVal;


pub struct TsumugiAntenna {
    pub parcel: Box<dyn TsumugiParcelInput + Send>,
    pub parceltype: TypeId,
    pub antennalifetime: TsumugiControllerItemLifeTime,
    ///受け取るTsumugiDistributorのparcel_nameと同じにしておく
    pub parcel_name: Option<String>,
    ///アンテナの名前を入れる。何も影響しないがデバッグがしやすくなる。
    pub antenna_name: Option<String>,
    pub current_state: TsumugiControllerItemState,
    pub antenna_application: TsumugiControllerApplication,
}

pub trait TsumugiFuture {
    fn poll(self: &mut Self, tct: &TsumugiPortalPlaneLocal) -> TsumugiControllerItemState;
}

pub trait TsumugiParcelInput {
    fn input_item(self: &mut Self, input_item: &mut Box<dyn Any + Send>, tct: &TsumugiPortalPlaneLocal) -> TsumugiControllerItemState;
}

pub trait TsumugiParcelOutput<T> {
    fn output_item(&self) -> &Box<T>;
}

impl TsumugiAntenna {
    pub fn lifetime(mut self, lifetime: TsumugiControllerItemLifeTime) -> Self {
        self.antennalifetime = lifetime;
        self
    }
    pub fn parcelname(mut self, name: impl ToString) -> Self {
        self.parcel_name = Some(name.to_string());
        self
    }
    pub fn displayname(mut self, name: impl ToString) -> Self {
        self.antenna_name = Some(name.to_string());
        self
    }
}

impl<T: 'static + Send + Clone> From<TsumugiParcelReceptor<T>> for TsumugiAntenna {
    fn from(tsumugi_parcel_receptor: TsumugiParcelReceptor<T>) -> Self {
        TsumugiAntenna {
            parcel: Box::from(tsumugi_parcel_receptor),
            parceltype: TypeId::of::<T>(),
            antennalifetime: TsumugiControllerItemLifeTime::Eternal,
            parcel_name: None,
            antenna_name: None,
            current_state: TsumugiControllerItemState::Untreated,
            antenna_application: TsumugiControllerApplication::New,
        }
    }
}

impl<T: 'static + Send + Clone> From<TsumugiParcelReceptorNoVal<T>> for TsumugiAntenna {
    fn from(tsumugi_parcel_receptor_noval: TsumugiParcelReceptorNoVal<T>) -> Self {
        TsumugiAntenna {
            parcel: Box::from(tsumugi_parcel_receptor_noval),
            parceltype: TypeId::of::<T>(),
            antennalifetime: TsumugiControllerItemLifeTime::Eternal,
            parcel_name: None,
            antenna_name: None,
            current_state: TsumugiControllerItemState::Untreated,
            antenna_application: TsumugiControllerApplication::New,
        }
    }
}

impl<T: 'static + Send + Clone> From<TsumugiParcelReceptorWithChannel<T>> for TsumugiAntenna {
    fn from(value: TsumugiParcelReceptorWithChannel<T>) -> Self {
        let parcel_name = value.parcel_name.clone();
        let antenna_name = value.antenna_name.clone();
        TsumugiAntenna {
            parcel: Box::from(value),
            parceltype: TypeId::of::<T>(),
            antennalifetime: TsumugiControllerItemLifeTime::Eternal,
            parcel_name: parcel_name,
            antenna_name: antenna_name,
            current_state: TsumugiControllerItemState::Untreated,
            antenna_application: TsumugiControllerApplication::New,
        }
    }
}

impl<T: 'static + Send + Clone> From<TsumugiParcelReceptorReturnValue<T>> for TsumugiAntenna {
    fn from(item: TsumugiParcelReceptorReturnValue<T>) -> Self {
        TsumugiAntenna {
            parcel: Box::from(item),
            parceltype: TypeId::of::<T>(),
            antennalifetime: TsumugiControllerItemLifeTime::Eternal,
            parcel_name: None,
            antenna_name: None,
            current_state: TsumugiControllerItemState::Untreated,
            antenna_application: TsumugiControllerApplication::New,
        }
    }
}
