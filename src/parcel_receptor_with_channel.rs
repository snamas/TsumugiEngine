use tsumugi_macro::TsumugiAnyTrait;
use std::sync::{  mpsc};
use crate::antenna::{TsumugiCurrentState, TsumugiAntenna,  AntennaLifeTime, TsumugiParcelInput};
use std::any::TypeId;
use std::sync::mpsc::{Sender, Receiver};
use crate::antenna_chain::TsumugiSpownReceiver;

#[derive(Clone)]
pub struct TsumugiParcelReceptorWithChannel<S: Send + Clone + TsumugiAnyTrait> {
    pub sender: Sender<S>,
    pub(crate) parcel_name: Option<String>,
    pub(crate) antenna_name: Option<String>,
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

impl<T: 'static + Send + Clone + TsumugiAnyTrait> TsumugiParcelReceptorWithChannel<T> {
    pub fn new()->TsumugiParcelReceptorWithChannel<T>{
        let (sender, _receiver): (Sender<T>, Receiver<T>) = mpsc::channel();
        TsumugiParcelReceptorWithChannel{
            sender: sender,
            parcel_name: None,
            antenna_name: None
        }
    }
    pub fn set_name(mut self, name:impl Into<String>) ->Self{
        self.antenna_name = Some(name.into());
        self
    }
}
impl<T: 'static + Send + Clone + TsumugiAnyTrait> TsumugiSpownReceiver for TsumugiParcelReceptorWithChannel<T> {
    type Output = Receiver<T>;

    fn spown_receiver(&mut self) -> Self::Output {
        let (sender, receiver): (Sender<T>, Receiver<T>) = mpsc::channel();
        self.sender = sender;
        return receiver;
    }
}

#[cfg(test)]
mod tests {
    use tsumugi_macro::{TsumugiAnyTrait,TsumugiAny};
    use std::any::{Any, TypeId};
    use crate::parcel_receptor_with_channel::TsumugiParcelReceptorWithChannel;
    use crate::antenna::TsumugiAntenna;

    #[derive(Clone,TsumugiAny)]
    struct Parcel {
        package: i32,
    }

    #[test]
    fn namecheck_some() {
        let recept = TsumugiParcelReceptorWithChannel::<Parcel>::new().set_name("test");
        let antenna:TsumugiAntenna = recept.into();
        assert_eq!(antenna.antenna_name, Some(String::from("test")));
    }

    #[test]
    fn namecheck_none() {
        let recept = TsumugiParcelReceptorWithChannel::<Parcel>::new();
        let antenna:TsumugiAntenna = recept.into();
        assert_eq!(antenna.antenna_name, None);
    }
}