use std::future::Future;
use std::sync::{Arc, Mutex, mpsc};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::sync::mpsc::{Sender, Receiver};
use std::any::{Any, TypeId};
use std::thread;
use std::ops::BitAnd;

struct ObjectA{
    input_item:Arc<Mutex<i32>>
}
impl ObjectA{
    fn spownticket(&self)-> ObjectATicket {
        ObjectATicket { receive_object: self.input_item.clone(), item: None, itemhash: TypeId::of::<Backet>() }
    }
    fn spownTsumugiTicket(&self,channnel:Sender<Box<dyn TsumugiFuture+ Send>>)-> TsumugiTicket {
        TsumugiTicket { receive_object: self.input_item.clone(), channel: channnel, item: None, baggettype: TypeId::of::<Backet>(), baggatlifetime: BaggetLifeTime::cold }
    }
}
enum BaggetLifeTime{
    shot,
    cold
}
struct ObjectATicket where{
    receive_object:Arc<Mutex<i32>>,
    item:Option<Box<Backet>>,
    itemhash:TypeId
}
struct TsumugiTicket{
    receive_object:Arc<Mutex<i32>>,
    channel:Sender<Box<dyn TsumugiFuture+ Send>>,
    item:Option<Box<Backet>>,
    baggettype:TypeId,
    baggatlifetime:BaggetLifeTime
}
trait TsumugiFuture:TsumugiTypeChacher{
    fn poll(self: &mut Self) -> Poll<()>;
    fn inputItem(self: &mut Self,inputItem: Box<dyn TsumugiTypeChacher+ Send>);
}
impl TsumugiFuture for ObjectATicket {
    fn poll(self: &mut Self) -> Poll<()> {
        if let Some(movaditem) = &self.item {
            let mut rec_obj = self.receive_object.lock().unwrap();
             *rec_obj += movaditem.package;
            dbg!(*rec_obj);
            return Poll::Ready(());
        }else {
            return Poll::Pending;
        }
    }

    fn inputItem(self: &mut Self, mut inputItem: Box<dyn TsumugiTypeChacher+ Send>) {
        let movaditem = inputItem.as_any().downcast_mut::<Backet>().unwrap();
        dbg!((*movaditem).package);
        let receive_item = unsafe {
            Box::from_raw(movaditem)
        };
        self.item = Option::from(receive_item);
        self.poll();
    }
}
impl TsumugiTypeChacher for ObjectATicket {
    fn typehash(&self)->TypeId{
        self.itemhash
    }
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

struct Backet{
    package:i32
}
trait TsumugiTypeChacher{
    fn as_any(&mut self) -> &mut dyn Any;
    fn typehash(&self)->TypeId;
}
impl TsumugiTypeChacher for Backet{
    fn typehash(&self) -> TypeId {
        self.type_id()
    }
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
fn main() {
    let spown_object_a = ObjectA{ input_item: Arc::new(Mutex::new(50))};
    let mut receive_ticket = spown_object_a.spownticket();
    let mut tumugiSetList:Vec<Box<dyn TsumugiFuture>> = Vec::new();
    let (mut txreceivebox, mut rxreceivebox): (Sender<Box<dyn TsumugiFuture+ Send>>, Receiver<Box<dyn TsumugiFuture+ Send>>) = mpsc::channel();
    txreceivebox.send(Box::new(receive_ticket));
    let (txsendbox, rxsendbox): (Sender<Box<dyn TsumugiTypeChacher+ Send>>, Receiver<Box<dyn TsumugiTypeChacher+ Send>>) = mpsc::channel();
    txsendbox.send(Box::new(Backet { package: 12 }));
    txsendbox.send(Box::new(Backet { package: 20 }));
    let handle = thread::spawn(move ||{

        let mut receiveList: Vec<_> =  rxreceivebox.try_iter().collect();
        let mut receiveItem: Box<dyn TsumugiFuture + Send> = receiveList.pop().unwrap();
        loop{
            let mut sendList: Vec<_> =  rxsendbox.try_iter().collect();
            while let Some(sendItem) = sendList.pop(){
                if receiveItem.typehash() == sendItem.typehash(){
                    receiveItem.inputItem(sendItem)
                }
            }
        }
        let mut sendList: Vec<_> =  rxsendbox.try_iter().collect();
        while let Some(sendItem) = sendList.pop(){
            if receiveItem.typehash() == sendItem.typehash(){
                receiveItem.inputItem(sendItem)
            }
        }
        println!("endthread!");

    });
    loop{
        let mut word = String::new();
        std::io::stdin().read_line(&mut word).ok();
        let answer = word.trim().to_string();
        if answer =="end"{
            break;

        }
        txsendbox.send(Box::new(Backet { package: 20 }));
    }
    println!("Hello, world!");
}
