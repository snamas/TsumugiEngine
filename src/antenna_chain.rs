use crate::antenna::{TsumugiAntenna, TsumugiParcelReceptorReturnValue, TsumugiCurrentState, TsumugiParcelReceptor, AntennaLifeTime};
use std::sync::{Arc, Mutex};
use std::fmt::DebugTuple;
use tsumugi_macro::{TsumugiAnyTrait,TsumugiAny,pack};
use std::any::{Any, TypeId};
use crate::parcel_receptor_with_channel::TsumugiParcelReceptorWithChannel;
use std::sync::mpsc::Receiver;
use std::mem;

macro_rules! antenna_chain {
     ( $( $x:tt ),*) => {
         TsumugiReceptorChain::from_receptor(($(
             $x.spown_receiver(),
         )*),vec![$(
             $x.into(),
         )*])
     };
}

pub enum TsumugiAntennaChainVecType {
    TsumugiAntenna(TsumugiAntenna),
    TsumugiAntennaChain(TsumugiAntennaChain),
}
pub trait TsumugiSpownReceiver{
    type output;
    fn spown_receiver(&mut self)->Self::output;
}
pub trait TsumugiReceptorChainTrait{
    fn execute_subscribe(&mut self);
}
impl <T: 'static + Send + Clone + TsumugiAnyTrait> From<TsumugiParcelReceptorWithChannel<T>> for TsumugiAntennaChainVecType {
    fn from(value: TsumugiParcelReceptorWithChannel<T>) -> Self {
        TsumugiAntennaChainVecType::TsumugiAntenna(TsumugiAntenna {
            parcel: Box::from(value),
            parceltype: TypeId::of::<T>(),
            parcellifetime: AntennaLifeTime::Once,
            parcel_name: None,
            current_state: TsumugiCurrentState::Untreated,
        })
    }
}
impl From<TsumugiAntennaChain> for TsumugiAntennaChainVecType {
    fn from(value: TsumugiAntennaChain) -> Self {
        TsumugiAntennaChainVecType::TsumugiAntennaChain(value)
    }
}

impl<T:Send+'static>  From<TsumugiReceptorChain<Box<T>>> for TsumugiAntennaChain {
    fn from(mut value: TsumugiReceptorChain<Box<T>>) -> Self {
        let tsumugiantennalist:Vec<TsumugiAntennaChainVecType> = mem::replace(&mut value.tsumugi_antenna_list, vec![]);
        TsumugiAntennaChain { antenna_chain: Box::new(value), tsumugi_antenna_list: tsumugiantennalist, parcellifetime: AntennaLifeTime::Eternal, parcel_name: None, current_state: TsumugiCurrentState::Untreated }
    }
}
pub struct TsumugiReceptorChain<T>{
    tsumugi_antenna_list:Vec<TsumugiAntennaChainVecType>,
    pub parcel: T,
    subscribe:Option<Box<dyn Fn(&mut T) -> TsumugiCurrentState + Send>>,
}

pub struct TsumugiAntennaChain{
    pub antenna_chain: Box<dyn TsumugiReceptorChainTrait+Send>,
    tsumugi_antenna_list:Vec<TsumugiAntennaChainVecType>,
    pub parcellifetime: AntennaLifeTime,
    pub parcel_name: Option<String>,
    pub current_state: TsumugiCurrentState,
}
impl <T:Send> TsumugiReceptorChain<T> {
    pub fn from_receptor(receivetuple: T, antenna:Vec<TsumugiAntennaChainVecType>) -> TsumugiReceptorChain<Box<T>> {
        TsumugiReceptorChain { tsumugi_antenna_list: antenna, parcel: Box::new(receivetuple), subscribe: None }
    }
    fn subscribe(mut self, func: Box<dyn Fn(&mut T) -> TsumugiCurrentState + Send>) ->Self{
        self.subscribe = Some(func);
        self
    }
}
impl <T:Send> TsumugiReceptorChainTrait for TsumugiReceptorChain<T>{
    fn execute_subscribe(&mut self) {
        if let Some(sub) = &self.subscribe{
            sub.as_ref()(&mut self.parcel);
        }
    }
}
impl <T:Send> TsumugiSpownReceiver for TsumugiReceptorChain<T>{
    type output = ();

    fn spown_receiver(&mut self) -> Self::output {
        todo!()
    }
}
impl TsumugiAntennaChain {
    //todo:あとで実装する。
    fn next(mut self)->TsumugiAntennaChain {
        TsumugiAntennaChain{
            antenna_chain: Box::new(TsumugiReceptorChain { tsumugi_antenna_list: vec![], parcel: Box::new(()), subscribe: None }),
            tsumugi_antenna_list: vec![self.into()],
            parcellifetime: AntennaLifeTime::Flash,
            parcel_name: None,
            current_state: TsumugiCurrentState::Untreated
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

pub fn test(){

    let mut tsumugi_pr = TsumugiParcelReceptorWithChannel::<Parcel>::new();
    let receivertsumugi_pr = tsumugi_pr.spown_receiver();
    let mut tb_pr = TsumugiParcelReceptorWithChannel::<Backet2>::new();
    let receivertb_pr = tb_pr.spown_receiver();
    let mut tsumugi_recept = TsumugiParcelReceptorWithChannel::<Backet2>::new();
    let receivertb_recepit = tsumugi_recept.spown_receiver();
    let mut tsumugi_antenna3:TsumugiAntenna = tsumugi_recept.into();


    let  a = TsumugiReceptorChain::from_receptor((receivertb_pr, receivertsumugi_pr), vec![tsumugi_pr.clone().into(), tb_pr.clone().into()]);
    let mut b = antenna_chain!(tb_pr,tsumugi_pr).subscribe(Box::new(|f|{
        let a =  &(**f).1;
        let b = &(**f).0;
        TsumugiCurrentState::Fulfilled
    }));
    let tsumugiantennalist:Vec<TsumugiAntennaChainVecType> = mem::replace(&mut b.tsumugi_antenna_list, vec![]);
    let mut c = TsumugiAntennaChain { antenna_chain: Box::new(b), tsumugi_antenna_list: tsumugiantennalist, parcellifetime: AntennaLifeTime::Eternal, parcel_name: None, current_state: TsumugiCurrentState::Untreated };
    c.antenna_chain.execute_subscribe();
}