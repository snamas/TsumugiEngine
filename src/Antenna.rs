use std::task::Poll;
use std::any::{TypeId, Any};
use crate::{TsumugiController};
use std::sync::{Mutex, Arc};
use tsumugi_any::TsumugiAnyTrait;

pub enum AntennalLifeTime {
    Shot,
    Cold,
    Lifetime(u32),
    LifeCount(u32),
    Update,
}

pub enum TsumugiCurrentState {
    Deny,
    Fulfilled,
    OnProgress,
}

pub struct TsumugiAntenna {
    pub parcel: Box<dyn TsumugiTypeConverter + Send>,
    pub parceltype: TypeId,
    pub parcellifetime: AntennalLifeTime,
    pub parcel_name: Option<String>,
    pub antenna_pack: Option<Arc<Mutex<TsumugiAntenna>>>,
}

pub struct TsumugiParcelReceipter<S: Send + Clone + TsumugiAnyTrait> {
    pub parcel: Box<S>,
    pub on_change: Box<dyn FnMut(&Box<S>) -> TsumugiCurrentState + Send>,
}

pub trait TsumugiFuture {
    fn poll(self: &mut Self) -> TsumugiCurrentState;
}

pub trait TsumugiTypeConverter {
    fn input_item(self: &mut Self, input_item: &mut Box<dyn TsumugiAnyTrait + Send>);
}

pub trait TsumugiAntennaTrait<T: 'static, S: 'static> {
    fn new(tsumugi_parcel_receipter: T) -> TsumugiAntenna;
}

impl<S: 'static + Send + Clone + TsumugiAnyTrait> TsumugiAntennaTrait<TsumugiParcelReceipter<S>, S> for TsumugiAntenna {
    fn new(tsumugi_parcel_receipter: TsumugiParcelReceipter<S>) -> TsumugiAntenna {
        TsumugiAntenna {
            parcel: Box::from(tsumugi_parcel_receipter),
            parceltype: TypeId::of::<S>(),
            parcellifetime: AntennalLifeTime::Shot,
            parcel_name: None,
            antenna_pack: None,
        }
    }
}

impl<T: Send + Clone + TsumugiAnyTrait> TsumugiFuture for TsumugiParcelReceipter<T> {
    fn poll(self: &mut Self) -> TsumugiCurrentState {
        self.on_change.as_mut()(&self.parcel)
    }
}

impl<T: 'static + TsumugiAnyTrait + Send + Clone> TsumugiTypeConverter for TsumugiParcelReceipter<T> {
    fn input_item(&mut self, input_item: &mut Box<dyn TsumugiAnyTrait + Send>) {
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
        self.poll();
    }
}

impl<T: 'static + Send + Clone + TsumugiAnyTrait> TsumugiParcelReceipter<T> {
    pub fn CreateTsumugiAntenna(self) -> TsumugiAntenna {
        TsumugiAntenna::new(self)
    }
}