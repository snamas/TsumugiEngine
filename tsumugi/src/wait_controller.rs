use std::sync::{Arc, Condvar, Mutex};
use crate::antenna_chain::TsumugiAntennaType;
use crate::controller::{TsumugiControllerItemLifeTime, TsumugiControllerItemState, TsumugiPortal};
use crate::signal::TsumugiSignal;

#[derive(Clone)]
/// あるタイミングまで待つための構造体
/// つむぎ内でwait_until_messageがくるまで待つよ
pub struct WaitObject {
    pair : Arc<(Mutex<bool>, Condvar)>
}

impl WaitObject {
    pub fn new() -> Self{
        WaitObject {
            pair : Arc::new((Mutex::new(false), Condvar::new()))
        }
    }
    fn spawn_wait_object(&self, tc:&TsumugiPortal, message:impl ToString){
        let pair = self.pair.clone();
        let signal:TsumugiAntennaType = TsumugiSignal::new(message).lifetime(TsumugiControllerItemLifeTime::Once).subscribe(Arc::new(move ||{
            let (lock, cvar) = &*pair;
            let mut started = lock.lock().unwrap();
            *started = true;
            cvar.notify_all();
            TsumugiControllerItemState::Fulfilled
        })).into();
        tc.global_channel_sender.wait(signal.into());
    }
    ///wait_until_messageをつむぎ内でsignalとして受け取るまで処理を止める。
    pub fn wait(&self, tc:&TsumugiPortal, wait_until_message:impl ToString){
        self.spawn_wait_object(&tc, wait_until_message);
        let (lock, cvar) = &*self.pair;
        let started = lock.lock().unwrap();
        let _ = cvar.wait_while(started, |started| !*started).unwrap();

    }
}