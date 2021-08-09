use std::task::Poll;
use std::any::{TypeId, Any};
use crate::{TsumugiController};
use std::sync::{Mutex, Arc};
use tsumugi_any::{TsumugiAnyTrait,TsumugiAny};
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

pub enum TsumugiAntennaChainVecType {
    A(TsumugiAntenna),
    AC(TsumugiAntennaChain),
}

pub struct TsumugiAntenna {
    pub parcel: Box<dyn TsumugiParcelInput + Send>,
    pub parceltype: TypeId,
    pub parcellifetime: AntennaLifeTime,
    pub parcel_name: Option<String>,
    pub current_state: TsumugiCurrentState,
    pub antenna_pack: Option<Arc<Mutex<TsumugiAntenna>>>,
}

pub struct TsumugiParcelReceipter<S: Send + Clone + TsumugiAnyTrait> {
    pub parcel: Box<S>,
    pub on_change: Box<dyn FnMut(&Box<S>) -> TsumugiCurrentState + Send>,
}
pub struct TsumugiParcelReceipterReturnValue<S: Send + Clone + TsumugiAnyTrait> {
    pub parcel: Mutex<Arc<S>>,
    pub on_change: Box<dyn FnMut(&Arc<S>) -> TsumugiCurrentState + Send>,
}
impl<S: 'static +  Send + Clone + TsumugiAnyTrait> TsumugiAnyTrait for TsumugiParcelReceipter<S>{
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

//todo:あとで実装する。
pub struct TsumugiAntennaChain {
    pub antenna_chain: Vec<TsumugiAntennaChainVecType>,

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
    fn newAntenna(tsumugi_parcel_receipter: T) -> TsumugiAntenna;
}
pub trait TsumugiAntennaTraitWithValue<T: 'static, S: 'static> {
    fn newAntennaWithValue(tsumugi_parcel_receipter: T) -> (TsumugiAntenna,Arc<S>);
}


impl<S: 'static + Send + Clone + TsumugiAnyTrait> TsumugiAntennaTrait<TsumugiParcelReceipter<S>, S> for TsumugiAntenna {
    fn newAntenna(tsumugi_parcel_receipter: TsumugiParcelReceipter<S>) -> TsumugiAntenna {
        TsumugiAntenna {
            parcel: Box::from(tsumugi_parcel_receipter),
            parceltype: TypeId::of::<S>(),
            parcellifetime: AntennaLifeTime::Once,
            parcel_name: None,
            current_state: TsumugiCurrentState::Pending,
            antenna_pack: None,
        }
    }

}

impl<T: Send + Clone + TsumugiAnyTrait> TsumugiFuture for TsumugiParcelReceipter<T> {
    fn poll(self: &mut Self) -> TsumugiCurrentState {
        self.on_change.as_mut()(&self.parcel)
    }
}

impl<T: 'static + TsumugiAnyTrait + Send + Clone> TsumugiParcelInput for TsumugiParcelReceipter<T> {
    fn input_item(&mut self, input_item: &mut Box<dyn TsumugiAnyTrait + Send>) -> TsumugiCurrentState {
        let movaditem = (*input_item).as_any().downcast_mut::<T>().unwrap();
        let mut receive_item = unsafe {
            Box::from_raw(movaditem)
        };
        let boxitem = receive_item.clone();
        let receive_item = receive_item as Box<dyn TsumugiAnyTrait + Send>;
        //この時点では、inputItemとreceive_itemは同じメモリアドレスの値となっている。
        //片方をforgetしてあげないとinputItemとreceive_item両方でメモリ解放が行われてしまう。
        std::mem::forget(receive_item);
        self.parcel = boxitem;
        self.poll()
    }
}
impl<T: 'static + Send + Clone + TsumugiAnyTrait> TsumugiParcelReceipter<T> {
    pub fn create_tsumugi_antenna(self:TsumugiParcelReceipter<T>) -> TsumugiAntenna {
        TsumugiAntenna::newAntenna(self)
    }
}

impl<S: 'static + Send + Clone + TsumugiAnyTrait+ Sync> TsumugiAntennaTraitWithValue<TsumugiParcelReceipterReturnValue<S>, S> for TsumugiAntenna {
    fn newAntennaWithValue(tsumugi_parcel_receipter_return_value: TsumugiParcelReceipterReturnValue<S>) -> (TsumugiAntenna,Arc<S>) {
        let parcelarc = tsumugi_parcel_receipter_return_value.parcel.lock().unwrap().clone();
        (TsumugiAntenna {
            parcel: Box::from(tsumugi_parcel_receipter_return_value),
            parceltype: TypeId::of::<S>(),
            parcellifetime: AntennaLifeTime::Once,
            parcel_name: None,
            current_state: TsumugiCurrentState::Pending,
            antenna_pack: None,
        },parcelarc)
    }

}
impl<T: Send + Clone + TsumugiAnyTrait> TsumugiFuture for TsumugiParcelReceipterReturnValue<T> {
    fn poll(self: &mut Self) -> TsumugiCurrentState {
        let parcelarc = self.parcel.lock().unwrap().clone();
        self.on_change.as_mut()(&parcelarc)
    }
}

impl<T: 'static + TsumugiAnyTrait + Send + Clone> TsumugiParcelInput for TsumugiParcelReceipterReturnValue<T> {
    fn input_item(&mut self, input_item: &mut Box<dyn TsumugiAnyTrait + Send>) -> TsumugiCurrentState {
        let movaditem = (*input_item).as_any().downcast_mut::<T>().unwrap();
        let mut receive_item = unsafe {
            Box::from_raw(movaditem)
        };
        let boxitem:Box<T> = receive_item.clone();
        let receive_item = receive_item as Box<dyn TsumugiAnyTrait + Send>;
        //この時点では、inputItemとreceive_itemは同じメモリアドレスの値となっている。
        //片方をforgetしてあげないとinputItemとreceive_item両方でメモリ解放が行われてしまう。
        std::mem::forget(receive_item);
        if let Ok(mut p) = self.parcel.lock(){
            *p  = Arc::from(boxitem);
        }
        self.poll()
    }
}