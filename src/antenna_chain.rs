use crate::antenna::{TsumugiAntenna, TsumugiCurrentState, AntennaLifeTime};
use std::sync::{  mpsc};
use tsumugi_macro::{TsumugiAnyTrait,TsumugiAny};
use std::any::{Any, TypeId};
use crate::parcel_receptor_with_channel::TsumugiParcelReceptorWithChannel;
use std::sync::mpsc::{Receiver, Sender};
use std::mem;
use std::collections::HashMap;
use crate::controller::TsumugiParcelHashList;

#[macro_export]
macro_rules! antenna_chain {
     ( $( $x:expr ),+) => {
         TsumugiReceptorChain::<_,()>::from_receptor(($(
             $x.spown_receiver(),
         )*),vec![$(
             $x.into(),
         )*])
     };
}

pub struct TsumugiReceptorChain<T,U=()>{
    tsumugi_antenna_list:Vec<TsumugiAntennaType>,
    pub parcel: T,
    subscribe:Option<Box<dyn Fn(&mut T) -> TsumugiCurrentState + Send>>,
    pub sender: Option<Sender<U>>,
    pub chain_name: Option<String>,
    chain_type:TsumugiAntennaChainType
}

pub struct TsumugiAntennaChain{
    pub antenna_chain: Box<dyn TsumugiReceptorChainTrait+Send>,
    pub(crate) tsumugi_antenna_list:Vec<TsumugiAntennaType>,
    pub parcellifetime: AntennaLifeTime,
    pub chain_name: Option<String>,
    pub current_state: TsumugiCurrentState,
    chain_type:TsumugiAntennaChainType
}
pub enum TsumugiAntennaType {
    TsumugiAntenna(TsumugiAntenna),
    TsumugiAntennaChain(TsumugiAntennaChain),
}
#[derive(Copy, Clone,Eq, PartialEq,Debug)]
pub enum TsumugiAntennaChainType {
    And,
    Next,
}
pub trait TsumugiSpownReceiver{
    type Output;
    fn spown_receiver(&mut self)->Self::Output;
}
pub trait TsumugiReceptorChainTrait{
    fn execute_subscribe(&mut self);
}
impl <T: 'static + Send + Clone + TsumugiAnyTrait> From<TsumugiParcelReceptorWithChannel<T>> for TsumugiAntennaType {
    fn from(value: TsumugiParcelReceptorWithChannel<T>) -> Self {
        TsumugiAntennaType::TsumugiAntenna(value.into())
    }
}
impl From<TsumugiAntennaChain> for TsumugiAntennaType {
    fn from(value: TsumugiAntennaChain) -> Self {
        TsumugiAntennaType::TsumugiAntennaChain(value)
    }
}
impl From<TsumugiAntenna> for TsumugiAntennaType {
    fn from(value: TsumugiAntenna) -> Self {
        TsumugiAntennaType::TsumugiAntenna(value)
    }
}

impl<T:Send+'static,U:Send+'static>  From<TsumugiReceptorChain<T,U>> for TsumugiAntennaChain {
    fn from(mut value: TsumugiReceptorChain<T,U>) -> Self {
        let tsumugiantennalist:Vec<TsumugiAntennaType> = std::mem::take(&mut value.tsumugi_antenna_list);
        let chainname = value.chain_name.clone();
        let chaintype = value.chain_type;
        TsumugiAntennaChain { antenna_chain: Box::new(value), tsumugi_antenna_list: tsumugiantennalist, parcellifetime: AntennaLifeTime::Eternal, chain_name: chainname, current_state: TsumugiCurrentState::Untreated, chain_type: chaintype }
    }
}

impl<T:Send+'static,U:Send+'static>  From<TsumugiReceptorChain<T,U>> for TsumugiAntennaType {
    fn from(mut value: TsumugiReceptorChain<T,U>) -> Self {
        TsumugiAntennaType::TsumugiAntennaChain(value.into())
    }
}

impl <T:Send+'static,U:Send+'static> TsumugiReceptorChain<T,U> {
    pub fn from_receptor(receivetuple: T, antenna:Vec<TsumugiAntennaType>) -> TsumugiReceptorChain<T> {
        TsumugiReceptorChain { tsumugi_antenna_list: antenna, parcel: receivetuple, subscribe: None, sender: None, chain_name: None, chain_type: TsumugiAntennaChainType::And }
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
impl <T:Send+'static,U:Send+'static> TsumugiReceptorChainTrait for TsumugiReceptorChain<T,U>{
    fn execute_subscribe(&mut self) {
        if let Some(sub) = &self.subscribe{
            sub.as_ref()(&mut self.parcel);
        }
    }
}
impl <T:Send+'static,U:Send+'static> TsumugiSpownReceiver for TsumugiReceptorChain<T,U>{
    type Output = Receiver<U>;
    fn spown_receiver(&mut self) -> Self::Output {
        let (sender, receiver): (Sender<U>, Receiver<U>) = mpsc::channel();
        self.sender=Some(sender);
        return receiver;
    }
}
impl <T:Send+'static,U:Send+'static> TsumugiReceptorChain<T,U> {
    //todo:あとで実装する。
    //todo:nectはparcelとTsumugiReceptorChainが同時に来たときに発動
    fn next<V:Send+'static>(mut self, mut receptor:impl TsumugiSpownReceiver<Output = Receiver<V>>+Into<TsumugiAntennaType>) ->TsumugiReceptorChain<(Receiver<U>,Receiver<V>),()> {
        let parcelreceiver = self.spown_receiver();
        let connectparcelreceiver = receptor.spown_receiver();
        TsumugiReceptorChain{
            tsumugi_antenna_list: vec![self.into(),receptor.into()],
            parcel: (parcelreceiver,connectparcelreceiver),
            subscribe: None,
            sender: None,
            chain_name: None,
            chain_type: TsumugiAntennaChainType::Next
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

#[cfg(test)]
mod tests {
    use tsumugi_macro::{TsumugiAny};
    use std::any::{Any};
    use crate::parcel_receptor_with_channel::TsumugiParcelReceptorWithChannel;
    use crate::antenna_chain::{TsumugiSpownReceiver, TsumugiAntennaChain, TsumugiAntennaType, TsumugiAntennaChainType};
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
    fn chainname(chainitem:&TsumugiAntennaType) -> ChainNameEnum {
        match chainitem {
            TsumugiAntennaType::TsumugiAntenna(value) => {
                ChainNameEnum::Antenna(value.antenna_name.clone())
            }
            TsumugiAntennaType::TsumugiAntennaChain(value) => {
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
        TsumugiReceptorChain::<_,()>::from_receptor((), vec![]);
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
        assert_eq!(chain2.chain_type,TsumugiAntennaChainType::And);
    }
    #[test]
    fn chaincheck_nextchain(){
        let mut tsumugi_pr = TsumugiParcelReceptorWithChannel::<Parcel>::new().set_name("parcel");
        let receivertsumugi_pr = tsumugi_pr.spown_receiver();
        let mut tb_pr = TsumugiParcelReceptorWithChannel::<Backet2>::new();
        let receivertb_pr = tb_pr.spown_receiver();
        let mut chain = antenna_chain!(tsumugi_pr.clone(),tb_pr.clone());
        let chain2 = chain.next(tsumugi_pr.clone());
        let antennachainname = chain2.tsumugi_antenna_list.iter().map(chainname).collect::<Vec<ChainNameEnum>>();
        assert_eq!(antennachainname,vec![ChainNameEnum::Antennachain(vec![ChainNameEnum::Antenna(Some(String::from("parcel"))),ChainNameEnum::Antenna(None)]),ChainNameEnum::Antenna(Some(String::from("parcel")))]);
        assert_eq!(chain2.chain_type,TsumugiAntennaChainType::Next);
        let antennachain2:TsumugiAntennaChain = chain2.into();
        assert_eq!(antennachain2.chain_type,TsumugiAntennaChainType::Next);

    }
}