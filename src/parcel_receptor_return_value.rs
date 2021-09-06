use tsumugi_macro::TsumugiAnyTrait;
use std::sync::{Mutex, Arc};
use crate::antenna::{TsumugiCurrentState, TsumugiAntenna, TsumugiParcelInput, TsumugiFuture, AntennaLifeTime};
use std::any::TypeId;

#[derive(Clone)]
pub struct TsumugiParcelReceptorReturnValue<T: Send + Clone + TsumugiAnyTrait> {
    pub parcel: Arc<Mutex<T>>,
    pub on_change: Option<Arc<dyn Fn(&TsumugiParcelReceptorReturnValue<T>) -> TsumugiCurrentState + Send+Sync>>,
}

pub trait TsumugiAntennaTraitWithValue<T: 'static> {
    type Receptor;
    fn new_antenna_with_value(tsumugi_parcel_receptor: Self::Receptor) -> (Arc<Mutex<T>>, TsumugiAntenna);
}
impl<T: 'static + Send + Clone + TsumugiAnyTrait+ Sync> TsumugiAntennaTraitWithValue<T> for TsumugiAntenna {
    type Receptor = TsumugiParcelReceptorReturnValue<T>;
    fn new_antenna_with_value(tsumugi_parcel_receptor_return_value: Self::Receptor) -> (Arc<Mutex<T>>,TsumugiAntenna) {
        (tsumugi_parcel_receptor_return_value.drop_tsumugi_parcel(),tsumugi_parcel_receptor_return_value.into())
    }
}
impl<T: Send + Clone + TsumugiAnyTrait> TsumugiFuture for TsumugiParcelReceptorReturnValue<T> {
    fn poll(self: &mut Self) -> TsumugiCurrentState {
        if let Some(fnc) =self.on_change.as_ref(){
            return fnc.as_ref()(&self);
        }
        TsumugiCurrentState::Deny
    }
}

impl<T: 'static + TsumugiAnyTrait + Send + Clone> TsumugiParcelInput for TsumugiParcelReceptorReturnValue<T> {
    fn input_item(&mut self, input_item: &mut Box<dyn TsumugiAnyTrait + Send>) -> TsumugiCurrentState {
        let movaditem = (*input_item).as_any().downcast_mut::<T>().unwrap();
        let mut receive_item = unsafe {
            Box::from_raw(movaditem)
        };
        let mut result = TsumugiCurrentState::Deny;
        if let Ok(mut p) = self.parcel.lock(){
            *p  = *receive_item.clone();
            result = TsumugiCurrentState::Fulfilled;
        }
        if result == TsumugiCurrentState::Fulfilled{
            result = self.poll();
        }
        let receive_item = receive_item as Box<dyn TsumugiAnyTrait + Send>;
        //この時点では、inputItemとreceive_itemは同じメモリアドレスの値となっている。
        //片方をforgetしてあげないとinputItemとreceive_item両方でメモリ解放が行われてしまう。
        std::mem::forget(receive_item);
        result
    }
}
impl<T: 'static + Send + Clone + TsumugiAnyTrait+ Sync> TsumugiParcelReceptorReturnValue<T> {
    pub fn drop_tsumugi_parcel(self:&TsumugiParcelReceptorReturnValue<T>) -> Arc<Mutex<T>> {
        self.parcel.clone()
    }
}