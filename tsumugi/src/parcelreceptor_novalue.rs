use std::any::Any;
use std::sync::Arc;
use crate::antenna::{TsumugiAntenna, TsumugiFuture, TsumugiParcelInput};
use crate::controller::{TsumugiController_threadlocal, TsumugiControllerItemState};

#[derive(Clone)]
pub struct TsumugiParcelReceptorNoVal<T: Send + Clone> {
    pub parcel: Option<Box<T>>,
    pub subscribe: Option<Arc<dyn Fn(&TsumugiParcelReceptorNoVal<T>, &TsumugiController_threadlocal) -> TsumugiControllerItemState + Send + Sync>>,
}

impl<T: Send + Clone> TsumugiFuture for TsumugiParcelReceptorNoVal<T> {
    //todo:ここparcelはOption型だが、絶対に値があることは分かっている
    fn poll(self: &mut Self, tct: &TsumugiController_threadlocal) -> TsumugiControllerItemState {
        if let Some(subscribe) = &self.subscribe {
            return subscribe.as_ref()(&self, tct);
        }
        TsumugiControllerItemState::Fulfilled
    }
}

impl<T: 'static + Send + Clone> TsumugiParcelReceptorNoVal<T> {
    ///TsumugiParcelReceptorNoVal::\<T\>::new()のようにして呼び出すよ。初期値が設定されていない時用のReceptor
    pub fn new() -> TsumugiParcelReceptorNoVal<T> {
        TsumugiParcelReceptorNoVal { parcel: None, subscribe: None }
    }
    pub fn subscribe(mut self, func: Arc<dyn Fn(&TsumugiParcelReceptorNoVal<T>) -> TsumugiControllerItemState + Send + Sync>) -> Self {
        self.subscribe = Some(Arc::new(move |parcel, tct| {
            func(parcel)
        }));
        self
    }
    pub fn subscribe_tc(mut self, func: Arc<dyn Fn(&TsumugiParcelReceptorNoVal<T>, &TsumugiController_threadlocal) -> TsumugiControllerItemState + Send + Sync>) -> Self {
        self.subscribe = Some(func);
        self
    }
    pub fn to_antenna(mut self) -> TsumugiAntenna {
        self.into()
    }
}

impl<T: 'static + Send + Clone> TsumugiParcelInput for TsumugiParcelReceptorNoVal<T> {
    fn input_item(&mut self, input_item: &mut Box<dyn Any + Send>, tct: &TsumugiController_threadlocal) -> TsumugiControllerItemState {
        let movaditem = (*input_item).downcast_mut::<T>().unwrap();
        let mut receive_item = unsafe {
            Box::from_raw(movaditem)
        };
        //ここ不安定要素
        self.parcel = Some(receive_item.clone());
        let receive_item = receive_item as Box<dyn Any + Send>;
        //この時点では、inputItemとreceive_itemは同じメモリアドレスの値となっている。
        //片方をforgetしてあげないとinputItemとreceive_item両方でメモリ解放が行われてしまう。
        std::mem::forget(receive_item);
        self.poll(tct)
    }
}