#[cfg(test)]
mod tests {
    use std::any::{Any, TypeId};
    use crate::parcel_receptor_with_channel::TsumugiParcelReceptorWithChannel;
    use crate::antenna_chain::{TsumugiSpawnReceiver, TsumugiAntennaType};
    use crate::antenna_chain::TsumugiReceptorChain;
    use std::collections::{HashMap, HashSet};
    use crate::controller::{DepotHashList, TsumugiAntennaChainHashList, TsumugiControllerApplication, TsumugiPortal, TsumugiChannelSenders, TsumugiObject, TsumugiParcelHashList, TsumugiControllerItemLifeTime, TsumugiControllerItemState, ControllLoopKitStruct, ThreadReceivers, TsumugiPortalPlaneLocal};
    use crate::distributor::{TsumugiParcelDistributor, TsumugiDistributor};
    use std::sync::mpsc::{Receiver, Sender};
    use std::time::{Instant};
    use std::sync::{mpsc, Mutex, Arc};
    use crate::parcel_receptor::TsumugiParcelReceptor;
    use crate::antenna::TsumugiAntenna;
    use std::sync::atomic::{AtomicBool, Ordering};
    use crate::signal::{Signal, TsumugiSignal};

    #[derive(Clone)]
    struct Parcel {
        package: i32,
    }

    #[derive(Clone)]
    struct ParcelStr {
        package: String,
    }

    #[derive(Clone)]
    struct Backet {
        package: i32,
    }

    impl ControllLoopKitStruct {
        fn new() -> (Self, TsumugiChannelSenders) {
            let (recept_channel_sender, receipt_channnel_receiver) = mpsc::channel();
            let (pickup_channel_sender, pickup_channnel_receiver) = mpsc::channel();
            let (object_sender, object_receiver) = mpsc::channel();
            let tsumugi_channel_senders = TsumugiChannelSenders { distributor_channel_sender: pickup_channel_sender, recept_channel_sender };
            let thread_receivers = ThreadReceivers {
                distributer: pickup_channnel_receiver,
                antenna: receipt_channnel_receiver,
                object: object_receiver,
            };
            let controll_test_kit_struct = ControllLoopKitStruct {
                thread_receivers: thread_receivers,
                inst_time: Instant::now(),
                depot_hashmap_typeof: DepotHashList { antenna_hashmap: HashMap::new(), signal_hashmap: HashMap::new() },
                antennachain_hashmap: TsumugiAntennaChainHashList { receipt_list: Vec::new() },
                object_list: vec![],
            };
            (controll_test_kit_struct, tsumugi_channel_senders)
        }
        fn checkhashmap(&mut self, typeid: TypeId) -> &mut TsumugiParcelHashList {
            self.depot_hashmap_typeof.antenna_hashmap.get_mut(&typeid).unwrap()
        }
    }

    impl TsumugiPortalPlaneLocal {
        pub(crate) fn new(tcs: TsumugiChannelSenders) -> Self {
            let (object_sender, object_receiver) = mpsc::channel();
            TsumugiPortalPlaneLocal {
                tp: TsumugiPortal {
                    local_channel_sender: tcs.clone(),
                    global_channel_sender: tcs,
                    connect_tsumugi_controller: vec![],
                    global_connect_tsumugi_controller: Arc::new(Mutex::new(Default::default())),
                    tsumugi_controller_name: "".to_string(),
                    tsumugi_object_sender: object_sender,
                },
                tsumugi_object_vector: vec![],
            }
        }
        fn execute_tsumugi_nothread(&mut self, controll_loop_kit: &mut ControllLoopKitStruct) {
            self.thread_loop_antenna_parcel(controll_loop_kit);
        }
    }

    impl TsumugiParcelHashList {
        ///(pickup_list, pickup_list_withid, recept_list. recept_list_withid)の順番で出力する。
        fn hashListCount(&self) -> (usize, usize, usize, usize) {
            (self.pickup_list.len(), self.pickup_list_withid.len(), self.recept_list.len(), self.recept_list_withid.len())
        }
    }

    #[test]
    fn chain_parse_test() {
        let (mut controll_loop_kit, tsumugi_channel_senders) = ControllLoopKitStruct::new();
        let mut tc = TsumugiPortalPlaneLocal::new(tsumugi_channel_senders.clone());
        let mut tsumugi_pr = TsumugiParcelReceptorWithChannel::<Parcel>::new().antenna_name("pa");
        let mut tb_pr = TsumugiParcelReceptorWithChannel::<Backet>::new();
        let mut chain = crate::antenna_chain!(tsumugi_pr.clone(),tb_pr);
        chain.chain_name = Some("chain".into());
        let chain2 = crate::antenna_chain!(tsumugi_pr,chain);
        let mut antenna_hashmap_typeof = DepotHashList { antenna_hashmap: HashMap::new(), signal_hashmap: HashMap::new() };
        let mut antennachain_hashmap = TsumugiAntennaChainHashList { receipt_list: Vec::new() };
        //[antenna[parcel(pa),antenna[paecel(pa),Backet2(None)]]
        tc.antenna_chain_action(chain2.into(), &mut antenna_hashmap_typeof, &mut antennachain_hashmap);
        {
            let antenna_name = antenna_hashmap_typeof.antenna_hashmap.iter().map(|(typeid, antenna)| {
                antenna.recept_list.iter().map(|x| { x.antenna_name.clone() }).collect::<Vec<Option<String>>>()
            }).collect::<HashSet<Vec<Option<String>>>>();
            let mut check_antennavec = HashSet::new();
            check_antennavec.insert(vec![Some("pa".into()), Some("pa".into())]);
            check_antennavec.insert(vec![None]);
            assert_eq!(antenna_name, check_antennavec);
        }
        {
            let antenna_name = antennachain_hashmap.receipt_list.iter().map(|x| {
                x.chain_name.clone()
            }).collect::<Vec<Option<String>>>();
            let check_antennachainvec = vec![Some("chain".into()), None];
            assert_eq!(antenna_name, check_antennachainvec);
        }
    }

    #[test]
    fn parcelparse_test() {
        let (mut controll_loop_kit, tsumugi_channel_senders) = ControllLoopKitStruct::new();
        let mut tc = TsumugiPortalPlaneLocal::new(tsumugi_channel_senders.clone());
        let parcel = TsumugiParcelDistributor::new(Parcel { package: 1 });
        let mut parcel_name = TsumugiParcelDistributor::new(Parcel { package: 1 });
        parcel_name.parcel_name = Some("parcel_is_here".into());
        let mut backet_name = TsumugiParcelDistributor::new(Backet { package: 1 });
        backet_name.parcel_name = Some("backet_is_here".into());
        let parcelpack = vec![parcel, parcel_name, backet_name];
        let mut depot_hashmap_typeof = DepotHashList { antenna_hashmap: HashMap::new(), signal_hashmap: HashMap::new() };
        for parcel in parcelpack {
            tc.parcel_action(parcel, &mut depot_hashmap_typeof);
        }
        {
            let parcel_noname = depot_hashmap_typeof.antenna_hashmap.iter().map(|(_typeid, value)| {
                value.pickup_list.iter().map(|x| {
                    x.parceltype
                }).collect::<Vec<TypeId>>()
            }).collect::<HashSet<Vec<TypeId>>>();
            let mut parcel_typelist = HashSet::new();
            parcel_typelist.insert(vec![TypeId::of::<Parcel>()]);
            parcel_typelist.insert(vec![]);
            assert_eq!(parcel_noname, parcel_typelist);
        }
        {
            let parcel_name = depot_hashmap_typeof.antenna_hashmap.iter().map(|(_typeid, value)| {
                value.pickup_list_withid.iter().map(|(name, _parcel)| { name.clone() }).collect::<Vec<String>>()
            }).collect::<HashSet<Vec<String>>>();
            let mut parcel_typelist = HashSet::new();
            parcel_typelist.insert(vec!["parcel_is_here".into()]);
            parcel_typelist.insert(vec!["backet_is_here".into()]);
            assert_eq!(parcel_name, parcel_typelist);
        }
    }

    #[test]
    fn controller_nothread_test() {
        let (mut controll_loop_kit, tsumugi_channel_senders) = ControllLoopKitStruct::new();
        let mut tc = TsumugiPortalPlaneLocal::new(tsumugi_channel_senders.clone());
        {
            //parcelを送る（Once）
            let new_parcel = TsumugiParcelDistributor::new(Parcel { package: 10 });
            tsumugi_channel_senders.distributor_channel_sender.send(new_parcel.into());
            tc.execute_tsumugi_nothread(&mut controll_loop_kit);
            let hashmap = controll_loop_kit.checkhashmap(TypeId::of::<Parcel>());
            assert_eq!(hashmap.hashListCount(), (1, 0, 0, 0));
            assert_eq!(hashmap.pickup_list.get(0).unwrap().parcellifetime, TsumugiControllerItemLifeTime::Once);
        }
        let mut parcelpackage = Arc::new(Mutex::new(1));
        let mut parcelpack_clone = parcelpackage.clone();
        let tsumugi_pr = TsumugiParcelReceptor::new(Parcel { package: 0 }).subscribe(
            Arc::new(move |parcel| {
                *parcelpack_clone.lock().unwrap() += parcel.parcel.package;
                TsumugiControllerItemState::Fulfilled
            }));
        {
            //Antennaを送る（Eternal,反応）
            tsumugi_channel_senders.recept_channel_sender.send(tsumugi_pr.clone().into());
            tc.execute_tsumugi_nothread(&mut controll_loop_kit);
            let hashmap = controll_loop_kit.checkhashmap(TypeId::of::<Parcel>());
            assert_eq!(*parcelpackage.lock().unwrap(), 11);
            assert_eq!(hashmap.hashListCount(), (0, 0, 1, 0));
        }
        {
            //Antennaを送る（Flash,無反応）
            let mut tsumugi_antenna: TsumugiAntenna = tsumugi_pr.clone().into();
            tsumugi_antenna.antennalifetime = TsumugiControllerItemLifeTime::Flash;
            tsumugi_channel_senders.recept_channel_sender.send(tsumugi_antenna.into());
            tc.execute_tsumugi_nothread(&mut controll_loop_kit);
            let hashmap = controll_loop_kit.checkhashmap(TypeId::of::<Parcel>());
            assert_eq!(*parcelpackage.lock().unwrap(), 11);
            assert_eq!(hashmap.hashListCount(), (0, 0, 1, 0));
        }
        {
            //Antennaを送る（Once,無反応）
            let mut tsumugi_antenna: TsumugiAntenna = tsumugi_pr.clone().into();
            tsumugi_antenna.antennalifetime = TsumugiControllerItemLifeTime::Once;
            tsumugi_channel_senders.recept_channel_sender.send(tsumugi_antenna.into());
            tc.execute_tsumugi_nothread(&mut controll_loop_kit);
            let hashmap = controll_loop_kit.checkhashmap(TypeId::of::<Parcel>());
            assert_eq!(*parcelpackage.lock().unwrap(), 11);
            assert_eq!(hashmap.hashListCount(), (0, 0, 2, 0));
        }
        {
            //parcelを送る（Once）
            let new_parcel = TsumugiParcelDistributor::new(Parcel { package: 20 });
            tsumugi_channel_senders.distributor_channel_sender.send(new_parcel.into());
            tc.execute_tsumugi_nothread(&mut controll_loop_kit);
            let hashmap = controll_loop_kit.checkhashmap(TypeId::of::<Parcel>());
            assert_eq!(*parcelpackage.lock().unwrap(), 51);
            assert_eq!(hashmap.hashListCount(), (0, 0, 1, 0));
        }
        //parcelを0にリセット
        *parcelpackage.lock().unwrap() = 0;
        //コントローラーをリセット
        let (mut controll_loop_kit, tsumugi_channel_senders) = ControllLoopKitStruct::new();
        {
            //parcelを送る（LifeCount(2)）
            let mut new_parcel = TsumugiParcelDistributor::new(Parcel { package: 10 });
            new_parcel.parcellifetime = TsumugiControllerItemLifeTime::LifeCount(2);
            tsumugi_channel_senders.distributor_channel_sender.send(new_parcel.into());
            tc.execute_tsumugi_nothread(&mut controll_loop_kit);
            let hashmap = controll_loop_kit.checkhashmap(TypeId::of::<Parcel>());
            assert_eq!(hashmap.hashListCount(), (1, 0, 0, 0));
            assert_eq!(hashmap.pickup_list.get(0).unwrap().parcellifetime, TsumugiControllerItemLifeTime::LifeCount(2));
        }
        {
            //Antennaを送る（Flash,反応）
            let mut tsumugi_antenna: TsumugiAntenna = tsumugi_pr.clone().into();
            tsumugi_antenna.antennalifetime = TsumugiControllerItemLifeTime::Flash;
            tsumugi_channel_senders.recept_channel_sender.send(tsumugi_antenna.into());
            tc.execute_tsumugi_nothread(&mut controll_loop_kit);
            let hashmap = controll_loop_kit.checkhashmap(TypeId::of::<Parcel>());
            assert_eq!(*parcelpackage.lock().unwrap(), 10);
            assert_eq!(hashmap.hashListCount(), (1, 0, 0, 0));
            assert_eq!(hashmap.pickup_list.get(0).unwrap().parcellifetime, TsumugiControllerItemLifeTime::LifeCount(1));
        }
        {
            //Antennaを送る（LifeCount(2),反応,Lifecycle(2),反応）ParcelはⅠ回目でLifeCountを消費して終了
            let mut tsumugi_antenna: TsumugiAntenna = tsumugi_pr.clone().into();
            tsumugi_antenna.antennalifetime = TsumugiControllerItemLifeTime::LifeCount(2);
            tsumugi_channel_senders.recept_channel_sender.send(tsumugi_antenna.into());
            let mut tsumugi_antenna: TsumugiAntenna = tsumugi_pr.clone().into();
            tsumugi_antenna.antennalifetime = TsumugiControllerItemLifeTime::Lifecycle(2);
            tsumugi_channel_senders.recept_channel_sender.send(tsumugi_antenna.into());
            tc.execute_tsumugi_nothread(&mut controll_loop_kit);
            let hashmap = controll_loop_kit.checkhashmap(TypeId::of::<Parcel>());
            assert_eq!(*parcelpackage.lock().unwrap(), 20);
            assert_eq!(hashmap.hashListCount(), (0, 0, 2, 0));
            assert_eq!(hashmap.recept_list.get(0).unwrap().antennalifetime, TsumugiControllerItemLifeTime::LifeCount(1));
            assert_eq!(hashmap.recept_list.get(1).unwrap().antennalifetime, TsumugiControllerItemLifeTime::Lifecycle(1));
        }
        {
            //Antennaを送る（renew）
            let mut tsumugi_antenna: TsumugiAntenna = tsumugi_pr.clone().into();
            tsumugi_antenna.antennalifetime = TsumugiControllerItemLifeTime::LifeCount(3);
            tsumugi_antenna.antenna_application = TsumugiControllerApplication::Renew;
            tsumugi_channel_senders.recept_channel_sender.send(tsumugi_antenna.into());
            tc.execute_tsumugi_nothread(&mut controll_loop_kit);
            let hashmap = controll_loop_kit.checkhashmap(TypeId::of::<Parcel>());
            assert_eq!(*parcelpackage.lock().unwrap(), 20);
            assert_eq!(hashmap.hashListCount(), (0, 0, 1, 0));
            assert_eq!(hashmap.recept_list.get(0).unwrap().antennalifetime, TsumugiControllerItemLifeTime::LifeCount(3));
        }
    }

    #[test]
    fn controller_antennachain_nothread_test() {
        let (mut controll_loop_kit, tsumugi_channel_senders) = ControllLoopKitStruct::new();
        let mut tc = TsumugiPortalPlaneLocal::new(tsumugi_channel_senders.clone());
        let mut parcelpackage = Arc::new(Mutex::new("NoMessage".to_string()));
        {
            //AntennaChainをつくる。AntennaChain[ParcelStr("pr"),AntennaChain[tsumugi_pr,tb_pr]]
            let mut tsumugi_pr = TsumugiParcelReceptorWithChannel::<ParcelStr>::new();
            let mut tb_pr = TsumugiParcelReceptorWithChannel::<Backet>::new();
            let mut chain = crate::antenna_chain!(tsumugi_pr.clone(),tb_pr);
            let mut parcelpack_clone = parcelpackage.clone();
            let mut chain = chain.set_name("chain").subscribe(Box::new(move |(parcel, backet), send| {
                let mut p = parcel.try_iter();
                let mut b = backet.try_iter();
                if let (Some(pitem), Some(bitem)) = (p.next(), b.next()) {
                    *parcelpack_clone.lock().unwrap() = pitem.package;
                    send.clone().unwrap().send(());
                    return TsumugiControllerItemState::Fulfilled;
                }
                return TsumugiControllerItemState::Deny;
            }));
            let mut parcel_receptname = tsumugi_pr.recept_name("pr");
            let mut parcelpack_clone = parcelpackage.clone();
            let mut chain2bool = AtomicBool::new(false);
            let chain2 = crate::antenna_chain!(parcel_receptname,chain).subscribe(Box::new(move |(parcel, antenna_chain_recv), send| {
                let mut p = parcel.try_iter();
                let mut ac = antenna_chain_recv.try_iter();
                if let Some(acitem) = ac.next() {
                    chain2bool.store(true, Ordering::SeqCst);
                }
                if let (Some(pitem), true) = (p.next(), chain2bool.load(Ordering::SeqCst)) {
                    *parcelpack_clone.lock().unwrap() = pitem.package;
                    return TsumugiControllerItemState::Fulfilled;
                }
                return TsumugiControllerItemState::Deny;
            }));
            tsumugi_channel_senders.recept_channel_sender.send(chain2.into());
            tc.execute_tsumugi_nothread(&mut controll_loop_kit);
            let hashmap = controll_loop_kit.checkhashmap(TypeId::of::<ParcelStr>());
            assert_eq!(hashmap.hashListCount(), (0, 0, 1, 1));
            dbg!(&controll_loop_kit.depot_hashmap_typeof.antenna_hashmap.get_mut(&TypeId::of::<ParcelStr>()).unwrap().recept_list_withid.get_mut(&"pr".to_string()).unwrap().get(0).unwrap().parcel_name.as_ref().unwrap());
            let hashmap = controll_loop_kit.checkhashmap(TypeId::of::<Backet>());
            assert_eq!(hashmap.hashListCount(), (0, 0, 1, 0));
        }
        {
            //parcelを送る（Once）
            let mut new_parcel = TsumugiParcelDistributor::new(ParcelStr { package: "ParcelIsReceived".to_string() });
            tsumugi_channel_senders.distributor_channel_sender.send(new_parcel.into());
            tc.execute_tsumugi_nothread(&mut controll_loop_kit);
            let hashmap = controll_loop_kit.checkhashmap(TypeId::of::<ParcelStr>());
            assert_eq!(hashmap.hashListCount(), (0, 0, 1, 1));
            assert_eq!(*parcelpackage.lock().unwrap(), "NoMessage".to_string());
        }
        {
            //parcelとbacketを送る（Once）
            let mut new_parcel = TsumugiParcelDistributor::new(ParcelStr { package: "ParcelIsReceived".to_string() });
            tsumugi_channel_senders.distributor_channel_sender.send(new_parcel.into());
            let mut new_backet = TsumugiParcelDistributor::new(Backet { package: 51 });
            tsumugi_channel_senders.distributor_channel_sender.send(new_backet.into());
            tc.execute_tsumugi_nothread(&mut controll_loop_kit);
            let hashmap = controll_loop_kit.checkhashmap(TypeId::of::<ParcelStr>());
            assert_eq!(hashmap.hashListCount(), (0, 0, 1, 1));
            assert_eq!(*parcelpackage.lock().unwrap(), "ParcelIsReceived".to_string());
        }
        {
            //parcel("pr")を送る（Once）
            let mut new_parcel = TsumugiParcelDistributor::new(ParcelStr { package: "NamedParcelIsReceived".to_string() }).parcelname("pr");
            tsumugi_channel_senders.distributor_channel_sender.send(new_parcel.into());
            tc.execute_tsumugi_nothread(&mut controll_loop_kit);
            let hashmap = controll_loop_kit.checkhashmap(TypeId::of::<ParcelStr>());
            assert_eq!(*parcelpackage.lock().unwrap(), "NamedParcelIsReceived".to_string());
            assert_eq!(hashmap.hashListCount(), (0, 1, 1, 1));
        }
    }
}