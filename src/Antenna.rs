use std::task::Poll;
use std::any::{TypeId, Any};
use crate::{TsumugiController};
use std::sync::{Mutex, Arc};
use tsumugi_macro::{TsumugiAnyTrait,TsumugiAny};
use std::convert::TryInto;

pub enum AntennaLifeTime {
    Flash,
    Once,
    Eternal,
    Lifetime(u32),
    LifeCount(u32),
    Update,
}

#[derive(PartialEq)]
pub enum TsumugiCurrentState {
    Pending,
    Deny,
    Fulfilled,
    OnProgress,
}

pub enum TsumugiAntennaChainType {
    And,
    Or,
    Next,
    Not,
}


pub struct TsumugiAntenna {
    pub parcel: Box<dyn TsumugiParcelInput + Send>,
    pub parceltype: TypeId,
    pub parcellifetime: AntennaLifeTime,
    pub parcel_name: Option<String>,
    pub current_state: TsumugiCurrentState,
    pub antenna_pack: Option<Arc<Mutex<TsumugiAntenna>>>,
}

pub struct TsumugiParcelReceptor<S: Send + Clone + TsumugiAnyTrait> {
    pub parcel: Box<S>,
    pub on_change: Box<dyn Fn(&TsumugiParcelReceptor<S>) -> TsumugiCurrentState + Send>,
}

#[derive(Clone)]
pub struct TsumugiParcelReceptorReturnValue<S: Send + Clone + TsumugiAnyTrait> {
    pub parcel: Arc<Mutex<S>>,
    pub on_change: Option<Arc<dyn Fn(&TsumugiParcelReceptorReturnValue<S>) -> TsumugiCurrentState + Send+Sync>>,
}
impl<S: 'static +  Send + Clone + TsumugiAnyTrait> TsumugiAnyTrait for TsumugiParcelReceptor<S>{
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
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
pub trait TsumugiAntennaTrait<T: 'static, S: 'static> {
    fn new_antenna(tsumugi_parcel_Receptor: T) -> TsumugiAntenna;
}
pub trait TsumugiAntennaTraitWithValue<T: 'static, S: 'static> {
    fn new_antenna_with_value(tsumugi_parcel_Receptor: T) -> (Arc<Mutex<S>>,TsumugiAntenna);
}


impl<S: 'static + Send + Clone + TsumugiAnyTrait> TsumugiAntennaTrait<TsumugiParcelReceptor<S>, S> for TsumugiAntenna {
    fn new_antenna(tsumugi_parcel_Receptor: TsumugiParcelReceptor<S>) -> TsumugiAntenna {
        TsumugiAntenna {
            parcel: Box::from(tsumugi_parcel_Receptor),
            parceltype: TypeId::of::<S>(),
            parcellifetime: AntennaLifeTime::Once,
            parcel_name: None,
            current_state: TsumugiCurrentState::Pending,
            antenna_pack: None,
        }
    }

}

impl<T: Send + Clone + TsumugiAnyTrait> TsumugiFuture for TsumugiParcelReceptor<T> {
    fn poll(self: &mut Self) -> TsumugiCurrentState {
        self.on_change.as_ref()(&self)
    }
}

impl<T: 'static + TsumugiAnyTrait + Send + Clone> TsumugiParcelInput for TsumugiParcelReceptor<T> {
    fn input_item(&mut self, input_item: &mut Box<dyn TsumugiAnyTrait + Send>) -> TsumugiCurrentState {
        let movaditem = (*input_item).as_any().downcast_mut::<T>().unwrap();
        let mut receive_item = unsafe {
            Box::from_raw(movaditem)
        };
        *self.parcel = *receive_item.clone();
        let receive_item = receive_item as Box<dyn TsumugiAnyTrait + Send>;
        //この時点では、inputItemとreceive_itemは同じメモリアドレスの値となっている。
        //片方をforgetしてあげないとinputItemとreceive_item両方でメモリ解放が行われてしまう。
        std::mem::forget(receive_item);
        self.poll()
    }
}
impl<T: 'static + Send + Clone + TsumugiAnyTrait> TsumugiParcelReceptor<T> {
    pub fn create_tsumugi_antenna(self:TsumugiParcelReceptor<T>) -> TsumugiAntenna {
        TsumugiAntenna::new_antenna(self)
    }
}

impl<S: 'static + Send + Clone + TsumugiAnyTrait+ Sync> TsumugiAntennaTraitWithValue<TsumugiParcelReceptorReturnValue<S>, S> for TsumugiAntenna {
    fn new_antenna_with_value(tsumugi_parcel_receptor_return_value: TsumugiParcelReceptorReturnValue<S>) -> (Arc<Mutex<S>>,TsumugiAntenna) {
        (tsumugi_parcel_receptor_return_value.drop_tsumugi_parcel(),tsumugi_parcel_receptor_return_value.create_tsumugi_antenna())
    }

}
impl<T: Send + Clone + TsumugiAnyTrait> TsumugiFuture for TsumugiParcelReceptorReturnValue<T> {
    fn poll(self: &mut Self) -> TsumugiCurrentState {
        if let Some(fnc) =self.on_change.as_ref(){
            return fnc.as_ref()(&self);
        }
        return TsumugiCurrentState::Fulfilled;
    }
}

impl<T: 'static + TsumugiAnyTrait + Send + Clone> TsumugiParcelInput for TsumugiParcelReceptorReturnValue<T> {
    fn input_item(&mut self, input_item: &mut Box<dyn TsumugiAnyTrait + Send>) -> TsumugiCurrentState {
        let movaditem = (*input_item).as_any().downcast_mut::<T>().unwrap();
        let mut receive_item = unsafe {
            Box::from_raw(movaditem)
        };
        if let Ok(mut p) = self.parcel.lock(){
            *p  = *receive_item.clone();
        }
        let receive_item = receive_item as Box<dyn TsumugiAnyTrait + Send>;
        //この時点では、inputItemとreceive_itemは同じメモリアドレスの値となっている。
        //片方をforgetしてあげないとinputItemとreceive_item両方でメモリ解放が行われてしまう。
        std::mem::forget(receive_item);
        self.poll()
    }
}
impl<T: 'static + Send + Clone + TsumugiAnyTrait+ Sync> TsumugiParcelReceptorReturnValue<T> {
    pub fn create_tsumugi_antenna(self:TsumugiParcelReceptorReturnValue<T>) -> TsumugiAntenna {
        TsumugiAntenna {
            parcel: Box::from(self),
            parceltype: TypeId::of::<T>(),
            parcellifetime: AntennaLifeTime::Once,
            parcel_name: None,
            current_state: TsumugiCurrentState::Pending,
            antenna_pack: None,
        }
    }
    pub fn drop_tsumugi_parcel(self:&TsumugiParcelReceptorReturnValue<T>) -> Arc<Mutex<T>> {
        self.parcel.clone()
    }
}
impl<T: 'static + TsumugiAnyTrait + Send + Clone> From<TsumugiParcelReceptorReturnValue<T>> for TsumugiAntenna{
    fn from(item: TsumugiParcelReceptorReturnValue<T>) -> Self {
        TsumugiAntenna {
            parcel: Box::from(item),
            parceltype: TypeId::of::<T>(),
            parcellifetime: AntennaLifeTime::Once,
            parcel_name: None,
            current_state: TsumugiCurrentState::Pending,
            antenna_pack: None,
        }
    }
}