use tsumugi_macro::TsumugiAnyTrait;
use crate::antenna::{ TsumugiFuture, TsumugiParcelInput, TsumugiAntenna};
use std::sync::Arc;
use std::any::Any;
use crate::controller::TsumugiControllerItemState;

#[derive(Clone)]
pub struct TsumugiParcelReceptor<T: Send + Clone> {
    pub parcel: Box<T>,
    pub subscribe: Option<Arc<dyn Fn(&TsumugiParcelReceptor<T>) -> TsumugiControllerItemState + Send+Sync>>,
}

impl<T: Send + Clone> TsumugiFuture for TsumugiParcelReceptor<T> {
    fn poll(self: &mut Self) -> TsumugiControllerItemState {
        if let Some(subscribe) = &self.subscribe{
            return subscribe.as_ref()(&self);
        }
        TsumugiControllerItemState::Fulfilled
    }
}
impl <T: Send + Clone>TsumugiParcelReceptor<T>{
    pub fn new(parcel:T)->TsumugiParcelReceptor<T>{
        TsumugiParcelReceptor{ parcel: Box::new(parcel), subscribe: None }
    }
    pub fn subscribe(mut self, func: Arc<dyn Fn(&TsumugiParcelReceptor<T>) -> TsumugiControllerItemState + Send + Sync>) ->Self{
        self.subscribe = Some(func);
        self
    }
}

impl<T: 'static + Send + Clone> TsumugiParcelInput for TsumugiParcelReceptor<T> {
    fn input_item(&mut self, input_item: &mut Box<dyn Any + Send>) -> TsumugiControllerItemState {
        let movaditem = (*input_item).downcast_mut::<T>().unwrap();
        let mut receive_item = unsafe {
            Box::from_raw(movaditem)
        };
        *self.parcel = *receive_item.clone();
        let receive_item = receive_item as Box<dyn Any + Send>;
        //この時点では、inputItemとreceive_itemは同じメモリアドレスの値となっている。
        //片方をforgetしてあげないとinputItemとreceive_item両方でメモリ解放が行われてしまう。
        std::mem::forget(receive_item);
        self.poll()
    }
}