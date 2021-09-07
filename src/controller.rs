use std::any::{TypeId};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{mpsc, Arc, Mutex, Condvar};
use std::thread;
use std::thread::JoinHandle;
use std::collections::HashMap;
use crate::antenna::{TsumugiAntenna, TsumugiFuture, TsumugiCurrentState};
use crate::distributor::{TsumugiParcelDistributor, ParcelLifeTime};
use tsumugi_macro::{TsumugiAnyTrait};
use crate::antenna_chain::{TsumugiAntennaChain, TsumugiAntennaType};
use std::hash::Hash;
use crate::distributor::ParcelLifeTime::{Fulfilled, Once};
use crate::antenna::AntennaLifeTime::Flash;
use std::time::{Instant, Duration};

pub struct TsumugiController {
    pub local_channel_sender:TsumugiChannelSenders,
    pub global_channel_sender:TsumugiChannelSenders,
    connect_tsumugi_controller: Vec<String>,
    pub global_connect_tsumugi_controller: Arc<Mutex<HashMap<String,Box<TsumugiController>>>>,
    pub tsumugi_controller_name: String,
    tsumugi_object_vector: Vec<Box<dyn TsumugiObject + Send>>
}

#[derive(Clone)]
pub struct TsumugiChannelSenders{
    pub pickup_channel_sender: Sender<TsumugiParcelDistributor>,
    pub recept_channel_sender: Sender<TsumugiAntennaType>,

}
pub struct TsumugiParcelHashList{
    pickup_list_withid:HashMap<String, Vec<TsumugiParcelDistributor>>,
    receipt_list_withid:HashMap<String,Vec<TsumugiAntenna>>,
    pickup_list: Vec<TsumugiParcelDistributor>,
    receipt_list:Vec<TsumugiAntenna>
}
struct DepotHashList(HashMap<TypeId,TsumugiParcelHashList>);

pub struct TsumugiAntennaChainHashList{
    receipt_list:Vec<TsumugiAntennaChain>
}

pub trait TsumugiObject {
    fn on_create(&self,tc:&TsumugiController);
}
pub trait TsumugiControllerTrait {
    fn new(tsumuginame: String) -> Box<TsumugiController>;
    fn spown(self:&Box<Self>, tsumuginame: String) -> Box<TsumugiController>;
    fn set_object(&mut self, tsumugi_object_list: Vec<Box<dyn TsumugiObject + Send>>);
    fn execute_tsumugi_functions(self:&Box<Self>, tsumugi_functions:Vec<Box<dyn Fn(&Box<TsumugiController>) -> Box<TsumugiController>>>);
    fn execute_tsumugi_thread(&self, receipt_channnel_receiver: Receiver<TsumugiAntennaType>,pickup_channnel_receiver: Receiver<TsumugiParcelDistributor>) -> JoinHandle<()>;
}

fn chain_parse(chain_item:TsumugiAntennaType, depot_hashmap_typeof:&mut DepotHashList, antenna_chain_hashlist:&mut TsumugiAntennaChainHashList){
    match chain_item {
        TsumugiAntennaType::TsumugiAntenna(antenna) => {
            let tsumugi_hash_typesep = depot_hashmap_typeof.0.entry(antenna.parceltype).or_insert(TsumugiParcelHashList{
                pickup_list_withid: Default::default(),
                receipt_list_withid: Default::default(),
                pickup_list:vec![],
                receipt_list: vec![]
            });
            if let Some(parcel_name) = &antenna.parcel_name{
                tsumugi_hash_typesep.receipt_list_withid.entry(parcel_name.clone()).or_insert_with(Vec::new).push(antenna);
            }else{
                tsumugi_hash_typesep.receipt_list.push(antenna);
            }
        }
        TsumugiAntennaType::TsumugiAntennaChain(mut antenna_chain) => {
            let antenna_list = std::mem::take(&mut antenna_chain.tsumugi_antenna_list);
            for antenna_item in antenna_list{
                chain_parse(antenna_item, depot_hashmap_typeof, antenna_chain_hashlist);
            }
            antenna_chain_hashlist.receipt_list.push(antenna_chain);
        }
    }
}

fn parcel_parse(pickup_item:TsumugiParcelDistributor, depot_hashmap_typeof:&mut DepotHashList){
    let tsumugi_parcel_hash_list = TsumugiParcelHashList{
        pickup_list_withid: Default::default(),
        receipt_list_withid: Default::default(),
        pickup_list:vec![],
        receipt_list: vec![]
    };
    dbg!(pickup_item.parceltype);
    let tsumugi_hash_typesep = depot_hashmap_typeof.0.entry(pickup_item.parceltype).or_insert(tsumugi_parcel_hash_list);
    if let Some(parcel_name) = &pickup_item.parcel_name{
        tsumugi_hash_typesep.pickup_list_withid.entry(parcel_name.clone()).or_insert_with(Vec::new).push(pickup_item);
    }else{
        tsumugi_hash_typesep.pickup_list.push(pickup_item);
    }
}
impl TsumugiControllerTrait for TsumugiController {
    fn new(tsumuginame: String) -> Box<TsumugiController> {
        let (recept_channel_sender, receipt_channnel_receiver): (Sender<TsumugiAntennaType>, Receiver<TsumugiAntennaType>) = mpsc::channel();
        let (pickup_channel_sender, pickup_channnel_receiver): (Sender<TsumugiParcelDistributor>, Receiver<TsumugiParcelDistributor>) = mpsc::channel();
        let tsumugi_channel_senders = TsumugiChannelSenders{pickup_channel_sender,recept_channel_sender};
        let mut tsumugi_connect_list: Vec<String> = Vec::new();
        let mut tsumugi_object_list: Vec<Box<dyn TsumugiObject + Send>> = Vec::new();
        let mut tc = Box::new(TsumugiController {
            local_channel_sender: tsumugi_channel_senders.clone(),
            global_channel_sender: tsumugi_channel_senders,
            connect_tsumugi_controller: tsumugi_connect_list,
            global_connect_tsumugi_controller: Arc::new(Mutex::new(HashMap::new())),
            tsumugi_controller_name: tsumuginame,
            tsumugi_object_vector: tsumugi_object_list
        });
        tc.execute_tsumugi_thread(receipt_channnel_receiver,pickup_channnel_receiver);
        return tc;
    }

    fn spown(self:&Box<Self>, tsumuginame: String) -> Box<TsumugiController> {
        let mut tc = Self::new(tsumuginame);
        tc.global_channel_sender = self.global_channel_sender.clone();
        return tc;
    }

    fn set_object(&mut self, mut tsumugi_object_list: Vec<Box<dyn TsumugiObject + Send>>) {
        for tsumugi_object in &tsumugi_object_list {
            tsumugi_object.on_create(self);
        }
        self.tsumugi_object_vector.append(&mut tsumugi_object_list);
    }
    /// TsumugiController生成関数の配列を受け取ってそれを使ってTsumugiControllerを生成していくよ
    fn execute_tsumugi_functions(self:&Box<Self>, create_tsumugi_controller_funclist: Vec<Box<dyn Fn(&Box<TsumugiController>) -> Box<TsumugiController>>>) {
        for tsumugi_function in create_tsumugi_controller_funclist {
            let mut tc_new = tsumugi_function(self);
            self.global_connect_tsumugi_controller.lock().unwrap().insert(tc_new.tsumugi_controller_name.clone(), tc_new as Box<TsumugiController>);
        }
    }
    fn execute_tsumugi_thread(&self, receipt_channnel_receiver: Receiver<TsumugiAntennaType>, pickup_channnel_receiver: Receiver<TsumugiParcelDistributor>) -> JoinHandle<()> {

        thread::spawn(move || {
            //todo:うまいことロックを使いこなそうcondvarというやつをつかって
            //todo:あとhashを値の管理に使う。
            let mut tumugi_receipt_list: Vec<Box<dyn TsumugiFuture + Send>> = Vec::new();
            let mut pickup_list: Vec<Box<dyn TsumugiAnyTrait + Send>> = Vec::new();
            let condvar = Condvar::new();
            let mutex = Mutex::new(());
            let lock = mutex.lock().unwrap();
            //condvar.wait(lock).unwrap();
            let mut depot_hashmap_typeof = DepotHashList(HashMap::new());
            let mut antennachain_hashmap = TsumugiAntennaChainHashList { receipt_list: Vec::new() };
            let mut inst_time = Instant::now();
            loop {
                let mut receipt_iter = receipt_channnel_receiver.try_iter();
                for receive_item in receipt_iter {
                    chain_parse(receive_item, &mut depot_hashmap_typeof, &mut antennachain_hashmap);
                }
                let mut pickup_iter  = pickup_channnel_receiver.try_iter();
                for pickup_item in pickup_iter {
                    parcel_parse(pickup_item,&mut depot_hashmap_typeof);
                }
                //todo:ここ値の受け渡し処理を行なう場所
                for tsumugi_hash in depot_hashmap_typeof.0.iter_mut(){
                    for pickupitem in tsumugi_hash.1.pickup_list.iter_mut(){
                        for receiptitem in tsumugi_hash.1.receipt_list.iter_mut(){
                            receiptitem.current_state = receiptitem.parcel.input_item(&mut pickupitem.parcel);
                            if pickupitem.parcellifetime == Once{
                                pickupitem.parcellifetime = Fulfilled;
                            }
                        }
                        pickupitem.parcellifetime = match pickupitem.parcellifetime {
                            ParcelLifeTime::Flash => {Fulfilled}
                            ParcelLifeTime::Once => {Once}
                            ParcelLifeTime::Eternal => {ParcelLifeTime::Eternal}
                            ParcelLifeTime::Lifetime(mut x) => {
                                let opt_x = x.checked_sub(inst_time.elapsed());
                                match opt_x {
                                    None => {Fulfilled}
                                    Some(x) => {ParcelLifeTime::Lifetime(x)}
                                }
                            }
                            ParcelLifeTime::Lifecycle(mut x) => {
                                x = x.saturating_sub(1);
                                if x == 0{
                                    ParcelLifeTime::Fulfilled
                                }else {
                                    ParcelLifeTime::Lifecycle(x)
                                }
                            }
                            ParcelLifeTime::LifeCount(x) => {ParcelLifeTime::LifeCount(x)}
                            ParcelLifeTime::Update => {ParcelLifeTime::Fulfilled}
                            ParcelLifeTime::Fulfilled => {ParcelLifeTime::Fulfilled}
                        }
                    }
                    //todo:ここpickupitemの性質（変更だけ受け取りたい場合もあるよ
                    tsumugi_hash.1.receipt_list.retain(|x| x.current_state != TsumugiCurrentState::Fulfilled);
                    tsumugi_hash.1.pickup_list.retain(|x| x.parcellifetime != ParcelLifeTime::Fulfilled);
                }
                inst_time = Instant::now();
            }
        }
        )
    }
}
#[cfg(test)]
mod tests {
    use tsumugi_macro::{TsumugiAny};
    use std::any::{Any, TypeId};
    use crate::parcel_receptor_with_channel::TsumugiParcelReceptorWithChannel;
    use crate::antenna_chain::{TsumugiSpownReceiver};
    use crate::antenna_chain::TsumugiReceptorChain;
    use std::collections::{HashMap, HashSet};
    use crate::controller::{ DepotHashList, TsumugiAntennaChainHashList, chain_parse, parcel_parse};
    use crate::distributor::TsumugiParcelDistributor;

    #[derive(Clone,TsumugiAny)]
    struct Parcel {
        package: i32,
    }

    #[derive(Clone,TsumugiAny)]
    struct Backet {
        package: i32,
    }
    #[test]
    fn chain_parse_test(){
        let mut tsumugi_pr = TsumugiParcelReceptorWithChannel::<Parcel>::new().set_name("pa");
        let mut tb_pr = TsumugiParcelReceptorWithChannel::<Backet>::new();
        let mut chain = crate::antenna_chain!(tsumugi_pr.clone(),tb_pr);
        chain.chain_name = Some("chain".into());
        let chain2 = crate::antenna_chain!(tsumugi_pr,chain);
        let mut antenna_hashmap_typeof = DepotHashList(HashMap::new());
        let mut antennachain_hashmap = TsumugiAntennaChainHashList { receipt_list: Vec::new() };
        //[antenna[parcel(pa),antenna[paecel(pa),Backet2(None)]]
        chain_parse(chain2.into(), &mut antenna_hashmap_typeof, &mut antennachain_hashmap);
        {
            let antenna_name = antenna_hashmap_typeof.0.iter().map(|(typeid,antenna)|{
                antenna.receipt_list.iter().map(|x|{x.antenna_name.clone()}).collect::<Vec<Option<String>>>()
            }).collect::<HashSet<Vec<Option<String>>>>();
            let mut check_antennavec = HashSet::new();
            check_antennavec.insert(vec![Some("pa".into()),Some("pa".into())]);
            check_antennavec.insert(vec![None]);
            assert_eq!(antenna_name,check_antennavec);
        }
        {
            let antenna_name = antennachain_hashmap.receipt_list.iter().map(|x|{
                x.chain_name.clone()
            }).collect::<Vec<Option<String>>>();
            let check_antennachainvec = vec![Some("chain".into()),None];
            assert_eq!(antenna_name,check_antennachainvec);
        }
    }
    #[test]
    fn parcelparse_test(){
        let parcel = TsumugiParcelDistributor::new(Parcel{ package: 1 });
        let mut parcel_name = TsumugiParcelDistributor::new(Parcel{ package: 1 });
        parcel_name.parcel_name = Some("parcel_is_here".into());
        let mut backet_name = TsumugiParcelDistributor::new(Backet{ package: 1 });
        backet_name.parcel_name = Some("backet_is_here".into());
        let parcelpack = vec![parcel,parcel_name,backet_name];
        let mut depot_hashmap_typeof = DepotHashList(HashMap::new());
        for parcel in parcelpack{
            parcel_parse(parcel,&mut depot_hashmap_typeof);
        }
        {
            let parcel_noname = depot_hashmap_typeof.0.iter().map(|(_typeid, value)|{
                value.pickup_list.iter().map(|x|{
                    x.parceltype
                }).collect::<Vec<TypeId>>()
            }).collect::<HashSet<Vec<TypeId>>>();
            let mut parcel_typelist = HashSet::new();
            parcel_typelist.insert(vec![TypeId::of::<Parcel>()]);
            parcel_typelist.insert(vec![]);
            assert_eq!(parcel_noname,parcel_typelist);
        }
        {
            let parcel_name = depot_hashmap_typeof.0.iter().map(|(_typeid,value)|{
                value.pickup_list_withid.iter().map(|(name,_parcel)|{name.clone()}).collect::<Vec<String>>()
            }).collect::<HashSet<Vec<String>>>();
            let mut parcel_typelist = HashSet::new();
            parcel_typelist.insert(vec!["parcel_is_here".into()]);
            parcel_typelist.insert(vec!["backet_is_here".into()]);
            assert_eq!(parcel_name,parcel_typelist);
        }

    }

}