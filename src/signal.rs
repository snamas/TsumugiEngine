use std::sync::{Arc, mpsc};
use std::sync::mpsc::{Receiver, Sender};
use crate::controller::{TsumugiControllerItemLifeTime, TsumugiControllerItemState, TsumugiControllerApplication};

pub struct Signal();

#[derive(Clone)]
pub struct TsumugiSignal {
    pub signallifetime: TsumugiControllerItemLifeTime,
    pub signal_name: String,
    pub current_state: TsumugiControllerItemState,
    pub on_receive_signal: Option<Arc<dyn Fn() -> TsumugiControllerItemState + Send + Sync>>,
    pub sender: Option<Sender<Signal>>,
}

impl TsumugiSignal {
    fn new(name: impl ToString) -> Self {
        TsumugiSignal {
            signallifetime: TsumugiControllerItemLifeTime::Flash,
            signal_name: name.to_string(),
            current_state: TsumugiControllerItemState::Untreated,
            on_receive_signal: None,
            sender: None,
        }
    }
    fn spown_receiver(&mut self) -> Receiver<Signal> {
        let (recept_channel_sender, recept_channnel_receiver): (Sender<Signal>, Receiver<Signal>) = mpsc::channel();
        self.sender = Some(recept_channel_sender);
        recept_channnel_receiver
    }
    fn subscribe(mut self, func: Arc<dyn Fn() -> TsumugiControllerItemState + Send + Sync>) -> Self {
        self.on_receive_signal = Some(func);
        self
    }
    fn lifetime(mut self, antenna_life_time: TsumugiControllerItemLifeTime) -> Self {
        self.signallifetime = antenna_life_time;
        self
    }
}