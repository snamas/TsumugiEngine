use std::sync::{Mutex, Arc};
use crate::antenna::{TsumugiAntenna, TsumugiParcelInput, TsumugiFuture};
use std::any::{TypeId, Any};
use crate::controller::{TsumugiPortalPlaneLocal, TsumugiControllerItemState};

#[derive(Clone)]
pub struct TsumugiParcelReceptorReturnValue<T: Send + Clone> {
    pub parcel: Arc<Mutex<T>>,
    pub on_change: Option<Arc<dyn Fn(&TsumugiParcelReceptorReturnValue<T>) -> TsumugiControllerItemState + Send + Sync>>,
}

pub trait TsumugiAntennaTraitWithValue<T: 'static> {
    type Receptor;
    fn new_antenna_with_value(tsumugi_parcel_receptor: Self::Receptor) -> (Arc<Mutex<T>>, TsumugiAntenna);
}

impl<T: 'static + Send + Clone + Sync> TsumugiAntennaTraitWithValue<T> for TsumugiAntenna {
    type Receptor = TsumugiParcelReceptorReturnValue<T>;
    fn new_antenna_with_value(tsumugi_parcel_receptor_return_value: Self::Receptor) -> (Arc<Mutex<T>>, TsumugiAntenna) {
        (tsumugi_parcel_receptor_return_value.drop_tsumugi_parcel(), tsumugi_parcel_receptor_return_value.into())
    }
}

impl<T: Send + Clone> TsumugiFuture for TsumugiParcelReceptorReturnValue<T> {
    fn poll(self: &mut Self, tct: &TsumugiPortalPlaneLocal) -> TsumugiControllerItemState {
        if let Some(fnc) = self.on_change.as_ref() {
            return fnc.as_ref()(&self);
        }
        TsumugiControllerItemState::Deny
    }
}

impl<T: 'static + Send + Clone> TsumugiParcelInput for TsumugiParcelReceptorReturnValue<T> {
    fn input_item(&mut self, input_item: &mut Box<dyn Any + Send>, tct: &TsumugiPortalPlaneLocal) -> TsumugiControllerItemState {
        let movaditem = (*input_item).downcast_mut::<T>().unwrap();
        let mut receive_item = unsafe {
            Box::from_raw(movaditem)
        };
        let mut result = TsumugiControllerItemState::Deny;
        if let Ok(mut p) = self.parcel.lock() {
            *p = *receive_item.clone();
            result = TsumugiControllerItemState::Fulfilled;
        }
        if result == TsumugiControllerItemState::Fulfilled {
            result = self.poll(tct);
        }
        let receive_item = receive_item as Box<dyn Any + Send>;
        //この時点では、inputItemとreceive_itemは同じメモリアドレスの値となっている。
        //片方をforgetしてあげないとinputItemとreceive_item両方でメモリ解放が行われてしまう。
        std::mem::forget(receive_item);
        result
    }
}

impl<T: 'static + Send + Clone + Sync> TsumugiParcelReceptorReturnValue<T> {
    pub fn drop_tsumugi_parcel(self: &TsumugiParcelReceptorReturnValue<T>) -> Arc<Mutex<T>> {
        self.parcel.clone()
    }
}