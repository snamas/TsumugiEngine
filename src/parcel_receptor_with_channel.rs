use tsumugi_macro::TsumugiAnyTrait;
use std::sync::{Arc, Mutex, mpsc};
use crate::antenna::{TsumugiCurrentState, TsumugiAntenna, TsumugiAntennaTrait, TsumugiParcelReceptor, AntennaLifeTime, TsumugiFuture, TsumugiParcelInput};
use std::any::TypeId;
use std::sync::mpsc::{Sender, Receiver};

#[derive(Clone)]
pub struct TsumugiParcelReceptorWithChannel<S: Send + Clone + TsumugiAnyTrait> {
    pub sender: Sender<S>,
}
impl<T: 'static + TsumugiAnyTrait + Send + Clone> TsumugiParcelInput for TsumugiParcelReceptorWithChannel<T> {
    fn input_item(&mut self, input_item: &mut Box<dyn TsumugiAnyTrait + Send>) -> TsumugiCurrentState {
        let movaditem = (*input_item).as_any().downcast_mut::<T>().unwrap();
        let mut receive_item = unsafe {
            Box::from_raw(movaditem)
        };
        self.sender.send(*receive_item.clone());
        let receive_item = receive_item as Box<dyn TsumugiAnyTrait + Send>;
        //この時点では、inputItemとreceive_itemは同じメモリアドレスの値となっている。
        //片方をforgetしてあげないとinputItemとreceive_item両方でメモリ解放が行われてしまう。
        std::mem::forget(receive_item);
        TsumugiCurrentState::Fulfilled
    }
}

impl<T: 'static + Send + Clone + TsumugiAnyTrait> From<TsumugiParcelReceptorWithChannel<T>> for TsumugiAntenna {
    fn from(value: TsumugiParcelReceptorWithChannel<T>) -> Self {
        TsumugiAntenna {
            parcel: Box::from(value),
            parceltype: TypeId::of::<T>(),
            parcellifetime: AntennaLifeTime::Once,
            parcel_name: None,
            current_state: TsumugiCurrentState::Untreated,
        }
    }
}

impl<T: 'static + Send + Clone + TsumugiAnyTrait> TsumugiParcelReceptorWithChannel<T> {
    pub fn spown_receiver(&mut self) -> Receiver<T> {
        let (sender, receiver): (Sender<T>, Receiver<T>) = mpsc::channel();
        self.sender = sender;
        return receiver;
    }
    pub fn new()->TsumugiParcelReceptorWithChannel<T>{
        let (sender, receiver): (Sender<T>, Receiver<T>) = mpsc::channel();
        TsumugiParcelReceptorWithChannel{
            sender: sender
        }
    }
}