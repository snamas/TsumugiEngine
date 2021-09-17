use std::any::{TypeId, Any};
use tsumugi_macro::{TsumugiAnyTrait};
use crate::parcel_receptor_with_channel::TsumugiParcelReceptorWithChannel;
use crate::parcel_receptor::TsumugiParcelReceptor;
use crate::parcel_receptor_return_value::TsumugiParcelReceptorReturnValue;
use crate::controller::{TsumugiControllerApplication, TsumugiControllerItemState, TsumugiControllerItemLifeTime};


pub struct TsumugiAntenna {
    pub parcel: Box<dyn TsumugiParcelInput + Send>,
    pub parceltype: TypeId,
    pub antennalifetime: TsumugiControllerItemLifeTime,
    pub parcel_name: Option<String>,
    pub antenna_name: Option<String>,
    pub current_state: TsumugiControllerItemState,
    pub antenna_application: TsumugiControllerApplication,
}

pub trait TsumugiFuture {
    fn poll(self: &mut Self) -> TsumugiControllerItemState;
}

pub trait TsumugiParcelInput{
    fn input_item(self: &mut Self, input_item: &mut Box<dyn Any + Send>) -> TsumugiControllerItemState;
}

pub trait TsumugiParcelOutput<T> {
    fn output_item(&self) -> &Box<T>;
}
impl <T: 'static + Send + Clone> From<TsumugiParcelReceptor<T>> for TsumugiAntenna{
    fn from(tsumugi_parcel_receptor: TsumugiParcelReceptor<T>) -> Self {
        TsumugiAntenna {
            parcel: Box::from(tsumugi_parcel_receptor),
            parceltype: TypeId::of::<T>(),
            antennalifetime: TsumugiControllerItemLifeTime::Eternal,
            parcel_name: None,
            antenna_name: None,
            current_state: TsumugiControllerItemState::Untreated,
            antenna_application: TsumugiControllerApplication::New
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
            antenna_application: TsumugiControllerApplication::New
        }
    }
}

impl<T: 'static + Send + Clone> From<TsumugiParcelReceptorReturnValue<T>> for TsumugiAntenna{
    fn from(item: TsumugiParcelReceptorReturnValue<T>) -> Self {
        TsumugiAntenna {
            parcel: Box::from(item),
            parceltype: TypeId::of::<T>(),
            antennalifetime: TsumugiControllerItemLifeTime::Eternal,
            parcel_name: None,
            antenna_name: None,
            current_state: TsumugiControllerItemState::Untreated,
            antenna_application: TsumugiControllerApplication::New
        }
    }
}
