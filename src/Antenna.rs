use std::any::{TypeId};
use tsumugi_macro::{TsumugiAnyTrait};
use crate::parcel_receptor_with_channel::TsumugiParcelReceptorWithChannel;
use crate::parcel_receptor::TsumugiParcelReceptor;
use crate::parcel_receptor_return_value::TsumugiParcelReceptorReturnValue;

#[derive(Clone)]
pub enum AntennaLifeTime {
    Eternal,
    Flash,
    Once,
    Lifetime(u32),
    LifeCount(u32),
    Update,
}

#[derive(PartialEq,Clone)]
pub enum TsumugiCurrentState {
    Untreated,
    Pending,
    Deny,
    Fulfilled,
    OnProgress,
}
pub struct TsumugiAntenna {
    pub parcel: Box<dyn TsumugiParcelInput + Send>,
    pub parceltype: TypeId,
    pub parcellifetime: AntennaLifeTime,
    pub parcel_name: Option<String>,
    pub antenna_name: Option<String>,
    pub current_state: TsumugiCurrentState,
}

pub trait TsumugiFuture {
    fn poll(self: &mut Self) -> TsumugiCurrentState;
}

pub trait TsumugiParcelInput{
    fn input_item(self: &mut Self, input_item: &mut Box<dyn TsumugiAnyTrait + Send>) -> TsumugiCurrentState;
}

pub trait TsumugiParcelOutput<T> {
    fn output_item(&self) -> &Box<T>;
}
impl <T: 'static + Send + Clone + TsumugiAnyTrait> From<TsumugiParcelReceptor<T>> for TsumugiAntenna{
    fn from(tsumugi_parcel_receptor: TsumugiParcelReceptor<T>) -> Self {
        TsumugiAntenna {
            parcel: Box::from(tsumugi_parcel_receptor),
            parceltype: TypeId::of::<T>(),
            parcellifetime: AntennaLifeTime::Eternal,
            parcel_name: None,
            antenna_name: None,
            current_state: TsumugiCurrentState::Untreated,
        }
    }
}

impl<T: 'static + Send + Clone + TsumugiAnyTrait> From<TsumugiParcelReceptorWithChannel<T>> for TsumugiAntenna {
    fn from(value: TsumugiParcelReceptorWithChannel<T>) -> Self {
        let parcel_name = value.parcel_name.clone();
        let antenna_name = value.antenna_name.clone();
        TsumugiAntenna {
            parcel: Box::from(value),
            parceltype: TypeId::of::<T>(),
            parcellifetime: AntennaLifeTime::Eternal,
            parcel_name: parcel_name,
            antenna_name: antenna_name,
            current_state: TsumugiCurrentState::Untreated,
        }
    }
}

impl<T: 'static + TsumugiAnyTrait + Send + Clone> From<TsumugiParcelReceptorReturnValue<T>> for TsumugiAntenna{
    fn from(item: TsumugiParcelReceptorReturnValue<T>) -> Self {
        TsumugiAntenna {
            parcel: Box::from(item),
            parceltype: TypeId::of::<T>(),
            parcellifetime: AntennaLifeTime::Eternal,
            parcel_name: None,
            antenna_name: None,
            current_state: TsumugiCurrentState::Untreated,
        }
    }
}
