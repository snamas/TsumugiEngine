use std::future::Future;
use std::sync::{Arc, Mutex, mpsc};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::sync::mpsc::{Sender, Receiver};
use std::any::{Any, TypeId};
use std::thread;
use std::ops::BitAnd;
use tsumugiEngine::{TsumugiController, TsumugiControllerTrait, TsumugiFuture, TsumugiTypeChacher, TsumugiObject, TsumugiTypeConverter, TsumugiAntenna, ParcelLifeTime};
use std::rc::Rc;

struct ObjectA {
    input_item: Arc<Mutex<i32>>,
    input_item_local: Arc<Mutex<i32>>,
}

impl ObjectA {
    fn spowntsumugiantenna(&self) -> TsumugiAntenna {
        let itemlock = self.input_item.clone();
        TsumugiAntenna{
            parcel:Box::from(TsumugiParcelReceipter {
                parcel: Option::from(Box::from(Parcel { package: 0 })),
                on_change: Box::new(move |parcel|{
                    let mut item = itemlock.lock().unwrap();
                    *item += parcel.package;
                    dbg!(*item);
                    return Poll::Ready(());
                })
            }),
            parceltype: TypeId::of::<Parcel>(),
            parcellifetime: ParcelLifeTime::Shot,
            parcel_name: None,
            antenna_pack: None
        }
    }
}
impl TsumugiObject for ObjectA{
    fn on_create(&self, tc: &TsumugiController){
        let mut receive_ticket = self.spowntsumugiantenna();
        tc.global_channel_sender.receipt_channel_sender.send(*Box::new(receive_ticket));

    }
}

pub fn spown_object_controller(tc: &Box<TsumugiController>) -> Box<TsumugiController> {
    let mut newtc = tc.spown("tsumugiobject".to_string());

    newtc.set_object(vec![
        Box::new(ObjectA { input_item: Arc::new(Mutex::new(200)), input_item_local: Arc::new(Mutex::new(0)) }),
        Box::new(ObjectA { input_item: Arc::new(Mutex::new(500)), input_item_local: Arc::new(Mutex::new(0)) })
    ]);
    return newtc;
}
struct TsumugiParcelReceipter<S:Send+Clone> {
    parcel:Option<Box<S>>,
    on_change: Box<dyn FnMut(Box<S>) -> Poll<()> + Send>
}
impl<T:Send+Clone> TsumugiFuture for TsumugiParcelReceipter<T> {
    fn poll(self: &mut Self) -> Poll<()> {
        if let Some(movaditem) = self.parcel.take() {
            return self.on_change.as_mut()(movaditem);
        } else {
            return Poll::Pending;
        }
    }
}
impl<T: 'static + TsumugiTypeChacher + Send+Clone> TsumugiTypeConverter for TsumugiParcelReceipter<T> {
    fn input_item(&mut self, input_item: &mut Box<dyn TsumugiTypeChacher + Send>) {
        let movaditem = (*input_item).as_any().downcast_mut::<T>().unwrap();
        let mut receive_item = unsafe {
            Box::from_raw(movaditem)
        };
        let boxitem = receive_item.clone();
        let receive_item = receive_item as Box<dyn TsumugiTypeChacher + Send>;
        //この時点では、inputItemとreceive_itemは同じメモリアドレスの値となっている。
        //片方をforgetしてあげないとinputItemとreceive_item両方でメモリ解放が行われてしまう。
        std::mem::forget(receive_item);
        self.parcel = Option::from(boxitem);
        self.poll();
    }
}

fn newReceiver<T: 'static>()-> Box<dyn TsumugiFuture> {
    struct ObjectReceiver<S> {
        parcel:Option<Box<S>>,
        on_change: Box<dyn FnMut(Box<S>) -> Poll<()> + Send>
    }
    impl<T> TsumugiFuture for ObjectReceiver<T> {
        fn poll(self: &mut Self) -> Poll<()> {
            if let Some(movaditem) = self.parcel.take() {
                return self.on_change.as_mut()(movaditem);
            } else {
                return Poll::Pending;
            }
        }
    }
    impl<T: 'static + TsumugiTypeChacher + Send+Clone> TsumugiTypeConverter for ObjectReceiver<T> {
        fn input_item(&mut self, input_item: &mut Box<dyn TsumugiTypeChacher + Send>) {
            let movaditem = (*input_item).as_any().downcast_mut::<T>().unwrap();
            let mut receive_item = unsafe {
                Box::from_raw(movaditem)
            };
            let boxitem = receive_item.clone();
            let receive_item = receive_item as Box<dyn TsumugiTypeChacher + Send>;
            //この時点では、inputItemとreceive_itemは同じメモリアドレスの値となっている。
            //片方をforgetしてあげないとinputItemとreceive_item両方でメモリ解放が行われてしまう。
            std::mem::forget(receive_item);
            self.parcel = Option::from(boxitem);
            self.poll();
        }
    }
    Box::new(ObjectReceiver{ parcel: None, on_change: Box::new((|x:Box<T>|{return Poll::Ready(())})) })
}
#[derive(Clone)]
struct Parcel {
    package: i32,
}

impl TsumugiTypeChacher for Parcel {
    fn typehash(&self) -> TypeId {
        self.type_id()
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Clone)]
struct Backet2 {
    package: i32,
}

impl TsumugiTypeChacher for Backet2 {
    fn typehash(&self) -> TypeId {
        self.type_id()
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

fn main() {
    let mut tsumugiroot = TsumugiController::new("Tsumugi".to_string());
    tsumugiroot.execute_tsumugi_functions(vec![Box::new(spown_object_controller)]);
    tsumugiroot.global_channel_sender.pickup_channel_sender.send(Box::new(Parcel { package: 12 }));
    loop {
        let mut word = String::new();
        std::io::stdin().read_line(&mut word).ok();
        let answer = word.trim().to_string();
        if answer == "end" {
            break;
        }
        tsumugiroot.local_channel_sender.pickup_channel_sender.send(Box::new(Parcel { package: 13 }));
        tsumugiroot.global_connect_tsumugi_controller.lock().unwrap().get("tsumugiobject").unwrap().local_channel_sender.pickup_channel_sender.send(Box::new(Parcel { package: 14 }));
        tsumugiroot.global_connect_tsumugi_controller.lock().unwrap().get("tsumugiobject").unwrap().global_channel_sender.pickup_channel_sender.send(Box::new(Parcel { package: 15 }));

        tsumugiroot.local_channel_sender.pickup_channel_sender.send(Box::new(Backet2 { package: 16 }));
    }
    println!("Hello, world!");
}
