use std::future::Future;
use std::sync::{Arc, Mutex, mpsc};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::sync::mpsc::{Sender, Receiver};
use std::any::{Any, TypeId};
use std::thread;
use std::ops::BitAnd;
use tsumugiEngine::{TsumugiController, TsumugiControllerTrait, TsumugiFuture, TsumugiTypeChacher, TsumugiObject, TsumugiTypeConverter};
use std::rc::Rc;

struct ObjectA {
    input_item: Arc<Mutex<i32>>,
    input_item_local: Arc<Mutex<i32>>,
}

impl ObjectA {
    fn spownticket(&self) -> ObjectAntenna {
        ObjectAntenna { receive_object: self.input_item.clone(), item: None, itemhash: TypeId::of::<Parcel>() }
    }
    fn spownticketlocal(&self) -> ObjectAntenna {
        ObjectAntenna { receive_object: self.input_item_local.clone(), item: None, itemhash: TypeId::of::<Parcel>() }
    }
    fn spowntsumugiantenna(&self) -> TsumugiAntenna {
        let itemlock = self.input_item.clone();
        TsumugiAntenna{
            parcel: None,
            parceltype: TypeId::of::<Parcel>(),
            parcellifetime: ParcelLifeTime::Shot,
            parcel_name: None,
            antenna_pack: None,
            on_change: Box::new(move |parcel|{
                let mut item = itemlock.lock().unwrap();
                *item += parcel.package;
                dbg!(*item);
                return Poll::Ready(());
            })
        }
    }
}
impl TsumugiObject for ObjectA{
    fn on_create(&self, tc: &TsumugiController){
        let mut receive_ticket = self.spowntsumugiantenna();
        tc.global_channel_sender.receipt_channel_sender.send(Box::new(receive_ticket));

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

enum ParcelLifeTime {
    Shot,
    Cold,
    Lifetime(u32),
    LifeCount(u32),
    Update
}

struct ObjectAntenna {
    receive_object: Arc<Mutex<i32>>,
    item: Option<Box<Parcel>>,
    itemhash: TypeId,
}

struct ObjectReceiver {
    receive_object: Arc<Mutex<i32>>,
    item: Option<Box<Parcel>>,

}

struct TsumugiAntenna {
    parcel:Option<Box<Parcel>>,
    parceltype: TypeId,
    parcellifetime: ParcelLifeTime,
    parcel_name: Option<String>,
    antenna_pack:Option<Arc<Mutex<TsumugiAntenna>>>,
    on_change: Box<dyn FnMut(Box<Parcel>) -> Poll<()> + Send>
}
impl TsumugiFuture for ObjectAntenna {
    fn poll(self: &mut Self) -> Poll<()> {
        if let Some(movaditem) = self.item.take() {
            let mut rec_obj = self.receive_object.lock().unwrap();
            *rec_obj += movaditem.package;
            dbg!(*rec_obj);
            return Poll::Ready(());
        } else {
            return Poll::Pending;
        }
    }
}
impl TsumugiTypeConverter for ObjectAntenna {
    fn input_item(&mut self, input_item: &mut Box<dyn TsumugiTypeChacher + Send>) {
        let movaditem = (*input_item).as_any().downcast_mut::<Parcel>().unwrap();
        dbg!((*movaditem).package);
        let mut receive_item = unsafe {
            Box::from_raw(movaditem)
        };
        let boxitem = receive_item.clone();
        let optionitem = Option::from(boxitem);
        let receive_item = receive_item as Box<dyn TsumugiTypeChacher + Send>;
        //この時点では、inputItemとreceive_itemは同じメモリアドレスの値となっている。
        //片方をforgetしてあげないとinputItemとreceive_item両方でメモリ解放が行われてしまう。
        std::mem::forget(receive_item);
        self.item = optionitem;
        self.poll();
    }
}
impl TsumugiTypeChacher for ObjectAntenna {
    fn typehash(&self) -> TypeId {
        self.itemhash
    }
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl TsumugiFuture for TsumugiAntenna {
    fn poll(self: &mut Self) -> Poll<()> {
        if let Some(movaditem) = self.parcel.take() {
            return self.on_change.as_mut()(movaditem);
        } else {
            return Poll::Pending;
        }
    }
}
impl TsumugiTypeConverter for TsumugiAntenna {
    fn input_item(&mut self, input_item: &mut Box<dyn TsumugiTypeChacher + Send>) {
        let movaditem = (*input_item).as_any().downcast_mut::<Parcel>().unwrap();
        dbg!((*movaditem).package);
        let mut receive_item = unsafe {
            Box::from_raw(movaditem)
        };
        let boxitem = receive_item.clone();
        let optionitem = Option::from(boxitem);
        let receive_item = receive_item as Box<dyn TsumugiTypeChacher + Send>;
        //この時点では、inputItemとreceive_itemは同じメモリアドレスの値となっている。
        //片方をforgetしてあげないとinputItemとreceive_item両方でメモリ解放が行われてしまう。
        std::mem::forget(receive_item);
        self.parcel = optionitem;
        self.poll();
    }
}
impl TsumugiTypeChacher for TsumugiAntenna {
    fn typehash(&self) -> TypeId {
        self.parceltype
    }
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
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
