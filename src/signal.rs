use std::sync::{Arc, mpsc};
use std::sync::mpsc::{Receiver, Sender};
use crate::controller::{TsumugiControllerItemLifeTime, TsumugiControllerItemState};

type Signal = ();
#[derive(Clone)]
pub struct TsumugiSignal {
    pub parcellifetime: TsumugiControllerItemLifeTime,
    pub parcel_name: String,
    pub current_state: TsumugiControllerItemState,
    pub on_receive_signal: Option<Arc<dyn Fn() -> TsumugiControllerItemState + Send+Sync>>,
}
impl TsumugiSignal{
    fn new(name:impl ToString)->Self{
        TsumugiSignal{
            parcellifetime: TsumugiControllerItemLifeTime::Flash,
            parcel_name: name.to_string(),
            current_state: TsumugiControllerItemState::Untreated,
            on_receive_signal: None
        }
    }
    fn spown_receiver(&self)->Receiver<Signal>{
        todo!();
        let (recept_channel_sender, recept_channnel_receiver): (Sender<Signal>, Receiver<Signal>) = mpsc::channel();
        recept_channnel_receiver
    }
    fn subscribe(mut self, func:Arc<dyn Fn() -> TsumugiControllerItemState + Send+Sync>) ->Self{
        self.on_receive_signal = Some(func);
        self
    }
    fn lifetime(mut self, antenna_life_time: TsumugiControllerItemLifeTime) ->Self{
        self.parcellifetime = antenna_life_time;
        self
    }
}