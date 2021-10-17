use std::sync::{mpsc};
use crate::antenna::{TsumugiAntenna, TsumugiParcelInput};
use std::any::{TypeId, Any};
use std::sync::mpsc::{Sender, Receiver};
use crate::antenna_chain::TsumugiSpownReceiver;
use crate::controller::{TsumugiController_thread, TsumugiControllerItemState};

#[derive(Clone)]
pub struct TsumugiParcelReceptorWithChannel<S: Send + Clone> {
    pub sender: Sender<S>,
    pub(crate) parcel_name: Option<String>,
    pub(crate) antenna_name: Option<String>,
}

impl<T: 'static + Send + Clone> TsumugiParcelInput for TsumugiParcelReceptorWithChannel<T> {
    fn input_item(&mut self, input_item: &mut Box<dyn Any + Send>, tct: &TsumugiController_thread) -> TsumugiControllerItemState {
        let movaditem = (*input_item).downcast_mut::<T>().unwrap();
        let mut receive_item = unsafe {
            Box::from_raw(movaditem)
        };
        self.sender.send(*receive_item.clone());
        let receive_item = receive_item as Box<dyn Any + Send>;
        //この時点では、inputItemとreceive_itemは同じメモリアドレスの値となっている。
        //片方をforgetしてあげないとinputItemとreceive_item両方でメモリ解放が行われてしまう。
        std::mem::forget(receive_item);
        TsumugiControllerItemState::Fulfilled
    }
}

impl<T: 'static + Send + Clone> TsumugiParcelReceptorWithChannel<T> {
    pub fn new() -> TsumugiParcelReceptorWithChannel<T> {
        let (sender, _receiver): (Sender<T>, Receiver<T>) = mpsc::channel();
        TsumugiParcelReceptorWithChannel {
            sender: sender,
            parcel_name: None,
            antenna_name: None,
        }
    }
    pub fn antenna_name(mut self, name: impl ToString) -> Self {
        self.antenna_name = Some(name.to_string());
        self
    }
    pub fn recept_name(mut self, name: impl ToString) -> Self {
        self.parcel_name = Some(name.to_string());
        self
    }
}

impl<T: 'static + Send + Clone> TsumugiSpownReceiver for TsumugiParcelReceptorWithChannel<T> {
    type Output = Receiver<T>;

    fn spown_receiver(&mut self) -> Self::Output {
        let (sender, receiver): (Sender<T>, Receiver<T>) = mpsc::channel();
        self.sender = sender;
        receiver
    }
}

#[cfg(test)]
mod tests {
    use std::any::{Any, TypeId};
    use crate::parcel_receptor_with_channel::TsumugiParcelReceptorWithChannel;
    use crate::antenna::TsumugiAntenna;

    #[derive(Clone)]
    struct Parcel {
        package: i32,
    }

    #[test]
    fn namecheck_some() {
        let recept = TsumugiParcelReceptorWithChannel::<Parcel>::new().antenna_name("test");
        let antenna: TsumugiAntenna = recept.into();
        assert_eq!(antenna.antenna_name, Some(String::from("test")));
    }

    #[test]
    fn namecheck_none() {
        let recept = TsumugiParcelReceptorWithChannel::<Parcel>::new();
        let antenna: TsumugiAntenna = recept.into();
        assert_eq!(antenna.antenna_name, None);
    }
}