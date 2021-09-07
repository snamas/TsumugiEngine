use tsumugi_macro::TsumugiAnyTrait;
use crate::antenna::{TsumugiCurrentState, TsumugiFuture, TsumugiParcelInput, TsumugiAntenna};
use std::sync::Arc;

#[derive(Clone)]
pub struct TsumugiParcelReceptor<T: Send + Clone + TsumugiAnyTrait> {
    pub parcel: Box<T>,
    pub subscribe: Option<Arc<dyn Fn(&TsumugiParcelReceptor<T>) -> TsumugiCurrentState + Send+Sync>>,
}

impl<T: Send + Clone + TsumugiAnyTrait> TsumugiFuture for TsumugiParcelReceptor<T> {
    fn poll(self: &mut Self) -> TsumugiCurrentState {
        if let Some(subscribe) = &self.subscribe{
            return subscribe.as_ref()(&self);
        }
        TsumugiCurrentState::Fulfilled
    }
}
impl <T: Send + Clone + TsumugiAnyTrait>TsumugiParcelReceptor<T>{
    pub fn new(parcel:T)->TsumugiParcelReceptor<T>{
        TsumugiParcelReceptor{ parcel: Box::new(parcel), subscribe: None }
    }
    pub fn subscribe(mut self, func: Arc<dyn Fn(&TsumugiParcelReceptor<T>) -> TsumugiCurrentState + Send + Sync>) ->Self{
        self.subscribe = Some(func);
        self
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