use crate::antenna::{TsumugiAntenna, TsumugiCurrentState, AntennaLifeTime};
use std::sync::{  mpsc};
use tsumugi_macro::{TsumugiAnyTrait,TsumugiAny};
use std::any::{Any, TypeId};
use crate::parcel_receptor_with_channel::TsumugiParcelReceptorWithChannel;
use std::sync::mpsc::{Receiver, Sender};
use std::mem;

#[macro_export]
macro_rules! antenna_chain {
     ( $( $x:expr ),*) => {
         TsumugiReceptorChain::from_receptor(($(
             $x.spown_receiver(),
         )*),vec![$(
             $x.into(),
         )*])
     };
}

pub struct TsumugiReceptorChain<T,U=()>{
    tsumugi_antenna_list:Vec<TsumugiAntennaChainVecType>,
    pub parcel: T,
    subscribe:Option<Box<dyn Fn(&mut T) -> TsumugiCurrentState + Send>>,
    pub sender: Option<Sender<U>>,
    pub chain_name: Option<String>,
}

pub struct TsumugiAntennaChain{
    pub antenna_chain: Box<dyn TsumugiReceptorChainTrait+Send>,
    tsumugi_antenna_list:Vec<TsumugiAntennaChainVecType>,
    pub parcellifetime: AntennaLifeTime,
    pub chain_name: Option<String>,
    pub current_state: TsumugiCurrentState,
}
pub enum TsumugiAntennaChainVecType {
    TsumugiAntenna(TsumugiAntenna),
    TsumugiAntennaChain(TsumugiAntennaChain),
}
pub trait TsumugiSpownReceiver{
    type Output;
    fn spown_receiver(&mut self)->Self::Output;
}
pub trait TsumugiReceptorChainTrait{
    fn execute_subscribe(&mut self);
}
impl <T: 'static + Send + Clone + TsumugiAnyTrait> From<TsumugiParcelReceptorWithChannel<T>> for TsumugiAntennaChainVecType {
    fn from(value: TsumugiParcelReceptorWithChannel<T>) -> Self {
        TsumugiAntennaChainVecType::TsumugiAntenna(value.into())
    }
}
impl From<TsumugiAntennaChain> for TsumugiAntennaChainVecType {
    fn from(value: TsumugiAntennaChain) -> Self {
        TsumugiAntennaChainVecType::TsumugiAntennaChain(value)
    }
}

impl<T:Send+'static>  From<TsumugiReceptorChain<T>> for TsumugiAntennaChain {
    fn from(mut value: TsumugiReceptorChain<T>) -> Self {
        let tsumugiantennalist:Vec<TsumugiAntennaChainVecType> = std::mem::take(&mut value.tsumugi_antenna_list);
        let chainname = value.chain_name.clone();
        TsumugiAntennaChain { antenna_chain: Box::new(value), tsumugi_antenna_list: tsumugiantennalist, parcellifetime: AntennaLifeTime::Eternal, chain_name: chainname, current_state: TsumugiCurrentState::Untreated }
    }
}

impl<T:Send+'static>  From<TsumugiReceptorChain<T>> for TsumugiAntennaChainVecType {
    fn from(mut value: TsumugiReceptorChain<T>) -> Self {
        TsumugiAntennaChainVecType::TsumugiAntennaChain(value.into())
    }
}

impl <T:Send> TsumugiReceptorChain<T> {
    pub fn from_receptor(receivetuple: T, antenna:Vec<TsumugiAntennaChainVecType>) -> TsumugiReceptorChain<T> {
        TsumugiReceptorChain { tsumugi_antenna_list: antenna, parcel: receivetuple, subscribe: None, sender: None, chain_name: None }
    }
    fn subscribe(mut self, func: Box<dyn Fn(&mut T) -> TsumugiCurrentState + Send>) ->Self{
        self.subscribe = Some(func);
        self
    }
    fn set_name(mut self,name:impl Into<String>)->Self{
        self.chain_name = Some(name.into());
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
impl <T:Send,U:Send> TsumugiSpownReceiver for TsumugiReceptorChain<T,U>{
    type Output = Receiver<U>;
    fn spown_receiver(&mut self) -> Self::Output {
        let (sender, receiver): (Sender<U>, Receiver<U>) = mpsc::channel();
        self.sender=Some(sender);
        return receiver;
    }
}
impl TsumugiAntennaChain {
    //todo:あとで実装する。
    fn next(mut self)->TsumugiAntennaChain {
        TsumugiAntennaChain{
            antenna_chain: Box::new(TsumugiReceptorChain { tsumugi_antenna_list: vec![], parcel: Box::new(()), subscribe: None, sender: None, chain_name: None }),
            tsumugi_antenna_list: vec![self.into()],
            parcellifetime: AntennaLifeTime::Eternal,
            chain_name: None,
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
        let a =  &(*f).1;
        let b = &(*f).0;
        TsumugiCurrentState::Fulfilled
    }));
    let tsumugiantennalist:Vec<TsumugiAntennaChainVecType> = mem::replace(&mut b.tsumugi_antenna_list, vec![]);
    let mut c = TsumugiAntennaChain { antenna_chain: Box::new(b), tsumugi_antenna_list: tsumugiantennalist, parcellifetime: AntennaLifeTime::Eternal, chain_name: None, current_state: TsumugiCurrentState::Untreated };
    c.antenna_chain.execute_subscribe();
}
#[cfg(test)]
mod tests {
    use tsumugi_macro::{TsumugiAny};
    use std::any::{Any};
    use crate::parcel_receptor_with_channel::TsumugiParcelReceptorWithChannel;
    use crate::antenna_chain::{TsumugiSpownReceiver, TsumugiAntennaChain, TsumugiAntennaChainVecType};
    use crate::antenna_chain::TsumugiReceptorChain;

    #[derive(Clone,TsumugiAny)]
    struct Parcel {
        package: i32,
    }

    #[derive(Clone,TsumugiAny)]
    struct Backet2 {
        package: i32,
    }
    #[derive(Eq, PartialEq,Debug)]
    enum ChainNameEnum{
        Antenna(Option<String>),
        Antennachain(Vec<ChainNameEnum>),
    }
    fn chainname(chainitem:&TsumugiAntennaChainVecType) -> ChainNameEnum {
        match chainitem {
            TsumugiAntennaChainVecType::TsumugiAntenna(value) => {
                ChainNameEnum::Antenna(value.parcel_name.clone())
            }
            TsumugiAntennaChainVecType::TsumugiAntennaChain(value) => {
                let chain = value.tsumugi_antenna_list.iter().map(chainname).collect::<Vec<ChainNameEnum>>();
                ChainNameEnum::Antennachain(chain)
            }
        }
    }
    #[test]
    fn chaincheck_antenna_only(){
        let mut tsumugi_pr = TsumugiParcelReceptorWithChannel::<Parcel>::new().set_name("parcel");
        let receivertsumugi_pr = tsumugi_pr.spown_receiver();
        let mut tb_pr = TsumugiParcelReceptorWithChannel::<Backet2>::new();
        let receivertb_pr = tb_pr.spown_receiver();
        let chain = antenna_chain!(tsumugi_pr.clone(),tb_pr.clone());
        let antennachain:TsumugiAntennaChain = chain.into();
        let antennachainname = antennachain.tsumugi_antenna_list.iter().map(chainname).collect::<Vec<ChainNameEnum>>();
        assert_eq!(antennachainname,vec![ChainNameEnum::Antenna(Some(String::from("parcel"))),ChainNameEnum::Antenna(None)]);

        let chain2 = antenna_chain!(tsumugi_pr.clone(),tb_pr.clone());
        let chainname2 = chain2.tsumugi_antenna_list.iter().map(chainname).collect::<Vec<ChainNameEnum>>();
        assert_eq!(antennachainname,chainname2);
    }
    #[test]
    fn chaincheck_antennachain(){
        let mut tsumugi_pr = TsumugiParcelReceptorWithChannel::<Parcel>::new().set_name("parcel");
        let receivertsumugi_pr = tsumugi_pr.spown_receiver();
        let mut tb_pr = TsumugiParcelReceptorWithChannel::<Backet2>::new();
        let receivertb_pr = tb_pr.spown_receiver();
        let mut chain = antenna_chain!(tsumugi_pr.clone(),tb_pr.clone());
        let chain2 = antenna_chain!(tsumugi_pr.clone(),chain);
        let antennachainname = chain2.tsumugi_antenna_list.iter().map(chainname).collect::<Vec<ChainNameEnum>>();
        assert_eq!(antennachainname,vec![ChainNameEnum::Antenna(Some(String::from("parcel"))),ChainNameEnum::Antennachain(vec![ChainNameEnum::Antenna(Some(String::from("parcel"))),ChainNameEnum::Antenna(None)])]);
    }
}