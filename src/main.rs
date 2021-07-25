use std::future::Future;
use std::sync::{Arc, Mutex, mpsc};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::sync::mpsc::{Sender, Receiver};
use std::any::{Any, TypeId};
use std::thread;
use std::ops::BitAnd;
use tsumugiEngine::{TsumugiController, TsumugiControllerTrait, TsumugiFuture, TsumugiTypeChacher};

struct ObjectA {
    input_item: Arc<Mutex<i32>>,
}

impl ObjectA {
    fn spownticket(&self) -> ObjectATicket {
        ObjectATicket { receive_object: self.input_item.clone(), item: None, itemhash: TypeId::of::<Backet>() }
    }
    fn spownTsumugiTicket(&self, channnel: Sender<Box<dyn TsumugiFuture + Send>>) -> TsumugiTicket {
        TsumugiTicket { receive_object: self.input_item.clone(), channel: channnel, item: None, parceltype: TypeId::of::<Backet>(), parcellifetime: ParcelLifeTime::Shot }
    }
}

enum ParcelLifeTime {
    Shot,
    Cold,
    Lifetime(i32),
}

struct ObjectATicket where {
    receive_object: Arc<Mutex<i32>>,
    item: Option<Box<Backet>>,
    itemhash: TypeId,
}

struct TsumugiTicket {
    receive_object: Arc<Mutex<i32>>,
    channel: Sender<Box<dyn TsumugiFuture + Send>>,
    item: Option<Box<Backet>>,
    parceltype: TypeId,
    parcellifetime: ParcelLifeTime,
}

impl TsumugiFuture for ObjectATicket {
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

    fn input_item(&mut self, mut inputItem: Box<dyn TsumugiTypeChacher + Send>) {
        let movaditem = inputItem.as_any().downcast_mut::<Backet>().unwrap();
        dbg!((*movaditem).package);
        let mut receive_item = unsafe {
            //ここでメモリリークするぞc0000374
            Box::from_raw(movaditem)
        };
        let boxitem = receive_item.clone();
        let optionitem = Option::from(boxitem);
        let receive_item = receive_item as Box<dyn TsumugiTypeChacher + Send>;
        //この時点では、inputItemとreceive_itemは同じメモリアドレスの値となっている。
        std::mem::forget(inputItem);
        //片方をforgetしてあげないとinputItemとreceive_item両方でメモリ解放が行われてしまう。
        drop(receive_item);
        self.item = optionitem;
        self.poll();
    }
}

impl TsumugiTypeChacher for ObjectATicket {
    fn typehash(&self) -> TypeId {
        self.itemhash
    }
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Clone)]
struct Backet {
    package: i32,
}

impl TsumugiTypeChacher for Backet {
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
    let tsumugiroot = TsumugiController::new("Tsumugi".to_string());
    let tsumugiObject = tsumugiroot.spown("tobj".to_string());


    let spown_object_a = ObjectA { input_item: Arc::new(Mutex::new(50)) };
    let mut receive_ticket = spown_object_a.spownticket();
    tsumugiroot.receipt_channnel_sender.send(Box::new(receive_ticket));
    tsumugiroot.pickup_channnel_sender.send(Box::new(Backet { package: 12 }));
    loop {
        let mut word = String::new();
        std::io::stdin().read_line(&mut word).ok();
        let answer = word.trim().to_string();
        if answer == "end" {
            break;
        }
        tsumugiroot.pickup_channnel_sender.send(Box::new(Backet { package: 13 }));
        tsumugiObject.pickup_channnel_sender.send(Box::new(Backet { package: 14 }));
        tsumugiObject.global_pickup_channnel_sender.send(Box::new(Backet { package: 15 }));

        tsumugiroot.pickup_channnel_sender.send(Box::new(Backet2 { package: 16 }));
    }
    println!("Hello, world!");
}
