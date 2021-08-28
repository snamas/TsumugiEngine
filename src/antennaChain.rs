use crate::antenna::{TsumugiAntenna, TsumugiParcelReceptorReturnValue, TsumugiCurrentState, TsumugiParcelReceptor};
use std::sync::{Arc, Mutex};
use std::fmt::DebugTuple;
use tsumugi_macro::{TsumugiAnyTrait,TsumugiAny,pack};
use std::any::Any;

macro_rules! antenna_chain {
     ( $( $x:tt ),*) => {
         TsumugiAntennaChain::from(($(
             $x.drop_tsumugi_parcel(),
         )*),vec![$(
             $x.create_tsumugi_antenna(),
         )*])
     };
}

pub enum TsumugiAntennaChainVecType<T> {
    A(TsumugiAntenna),
    AC(TsumugiAntennaChain<T>),
}
#[derive(Debug)]
enum ChainType{
    Next,
    And,
    Or
}
pub struct TsumugiAntennaValue<S>{
    tsumugi_antenna:Vec<TsumugiAntenna>,
    parcel: S
}
//todo:あとで実装する。
pub struct TsumugiAntennaChain<T> {
    pub antenna_chain: TsumugiAntennaValue<T>,
    chaintype:ChainType,
    onChange:Option<Box<dyn Fn(&T) -> TsumugiCurrentState + Send>>
}

impl<T> TsumugiAntennaChain<T> {
    fn from(tuple: T,antenna:Vec<TsumugiAntenna>) ->TsumugiAntennaChain<T> {
        TsumugiAntennaChain{
            antenna_chain: TsumugiAntennaValue{ tsumugi_antenna: antenna, parcel: tuple },
            chaintype: ChainType::And,
            onChange: None
        }
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

fn test(){

    let mut tsumugi_pr = TsumugiParcelReceptorReturnValue {
        parcel: Arc::new(Mutex::from(Parcel { package: 0 })),
        on_change: Some(Arc::new(move |parcel| {
            dbg!(parcel.parcel.lock().unwrap().package);
            return TsumugiCurrentState::Pending;
        })),
    };
    let mut tb_pr = TsumugiParcelReceptorReturnValue {
        parcel: Arc::new(Mutex::from(Backet2 { package: 0 })),
        on_change: Some(Arc::new(move |parcel| {
            dbg!(parcel.parcel.lock().unwrap().package);
            return TsumugiCurrentState::Pending;
        })),
    };
    let (mut tsumugi_antenna, mut parcel)= (tsumugi_pr.clone().create_tsumugi_antenna(),tsumugi_pr.clone().drop_tsumugi_parcel());
    let (mut tsumugi_antenna2, mut parcel2)= (tb_pr.clone().create_tsumugi_antenna(),tb_pr.clone().drop_tsumugi_parcel());
    let  a = TsumugiAntennaChain::from((parcel,parcel2),vec![tsumugi_antenna,tsumugi_antenna2]);
    let b = antenna_chain!(tsumugi_pr,tb_pr);
    dbg!(b.chaintype);

}