use tsumugi_macro::TsumugiAnyTrait;
use crate::antenna::{TsumugiCurrentState, TsumugiFuture, TsumugiParcelInput, TsumugiAntenna};
use std::sync::Arc;

#[derive(Clone)]
pub struct TsumugiParcelReceptor<S: Send + Clone + TsumugiAnyTrait> {
    pub parcel: Box<S>,
    pub on_change: Arc<dyn Fn(&TsumugiParcelReceptor<S>) -> TsumugiCurrentState + Send+Sync>,
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