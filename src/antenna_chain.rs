use crate::antenna::{TsumugiAntenna, TsumugiParcelReceptorReturnValue, TsumugiCurrentState, TsumugiParcelReceptor};
use std::sync::{Arc, Mutex};
use std::fmt::DebugTuple;
use tsumugi_macro::{TsumugiAnyTrait,TsumugiAny,pack};
use std::any::Any;
use crate::parcel_receptor_with_channel::TsumugiParcelReceptorWithChannel;

macro_rules! antenna_chain {
     ( $( $x:tt ),*) => {
         TsumugiAntennaChain::from(($(
             $x.spown_receiver(),
         )*),vec![$(
             $x.into(),
         )*])
     };
}

pub enum TsumugiAntennaChainVecType<T> {
    A(TsumugiAntenna),
    AC(TsumugiAntennaChain<T>),
}
pub struct TsumugiAntennaValue<S>{
    tsumugi_antenna:Vec<TsumugiAntenna>,
    pub parcel: S
}
//todo:あとで実装する。
pub struct TsumugiAntennaChain<T> {
    pub antenna_chain: TsumugiAntennaValue<T>,
    on_set:Option<Box<dyn Fn(&mut T) -> TsumugiCurrentState + Send>>
}

impl<T:Send> TsumugiAntennaChain<T> {
    fn from(receivetuple: T,antenna:Vec<TsumugiAntenna>) ->TsumugiAntennaChain<T> {
        TsumugiAntennaChain{
            antenna_chain: TsumugiAntennaValue{ tsumugi_antenna: antenna, parcel: receivetuple },
            on_set: None
        }
    }
    fn on_set(mut self, func: Box<dyn Fn(&mut T) -> TsumugiCurrentState + Send>) ->Self{
        self.on_set = Some(func);
        self
    }
}

#[derive(Clone,TsumugiAny)]
struct Parcel {
    package: i32,
}

#[derive(Clone,TsumugiAny)]
struct Backet2 {
    package: i32,
}

pub fn test(){

    let mut tsumugi_pr = TsumugiParcelReceptorWithChannel::<Parcel>::new();
    let receivertsumugi_pr = tsumugi_pr.spown_receiver();
    let mut tb_pr = TsumugiParcelReceptorWithChannel::<Backet2>::new();
    let receivertb_pr = tb_pr.spown_receiver();
    let mut tsumugi_recept = TsumugiParcelReceptorWithChannel::<Backet2>::new();
    let receivertb_recepit = tsumugi_recept.spown_receiver();
    let mut tsumugi_antenna3:TsumugiAntenna = tsumugi_recept.into();


    let  a = TsumugiAntennaChain::from((receivertb_pr,receivertsumugi_pr),vec![tsumugi_pr.clone().into(),tb_pr.clone().into()]);
    let mut b = antenna_chain!(tb_pr,tsumugi_pr).on_set(Box::new(|f|{
        let a =  &f.1;
        let b = &f.0;
        TsumugiCurrentState::Fulfilled
    }));
    b.antenna_chain.parcel.0;
    let c = b.antenna_chain.tsumugi_antenna.pop();
}