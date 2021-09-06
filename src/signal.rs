use crate::antenna::{AntennaLifeTime, TsumugiCurrentState};
use std::sync::{Arc, mpsc};
use std::sync::mpsc::{Receiver, Sender};
type Signal = ();
#[derive(Clone)]
pub struct TsumugiSignal {
    pub parcellifetime: AntennaLifeTime,
    pub parcel_name: String,
    pub current_state: TsumugiCurrentState,
    pub on_receive_signal: Option<Arc<dyn Fn() -> TsumugiCurrentState + Send+Sync>>,
}
impl TsumugiSignal{
    fn spown_receiver()->Receiver<Signal>{
        let (recept_channel_sender, recept_channnel_receiver): (Sender<Signal>, Receiver<Signal>) = mpsc::channel();
        recept_channnel_receiver
    }
}