use std::any::{TypeId, Any};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::task::Poll;

trait TsumugiFuture:TsumugiTypeChacher{
    fn poll(self: &mut Self) -> Poll<()>;
    fn input_item(self: &mut Self, input_item: Box<dyn TsumugiTypeChacher+ Send>);
}
trait TsumugiTypeChacher{
    fn as_any(&mut self) -> &mut dyn Any;
    fn typehash(&self)->TypeId;
}
trait TsumugiObject{
    fn on_create(&self);
}
pub struct TsumugiController {
    pickup_list:Vec<Box<dyn TsumugiTypeChacher+ Send>>,
    pickup_channnel_receiver:Receiver<Box<dyn TsumugiTypeChacher+ Send>>,
    pickup_channnel_sender:Sender<Box<dyn TsumugiTypeChacher+ Send>>,
    receipt_list:Vec<Box<dyn TsumugiFuture+ Send>>,
    receipt_channnel_receiver:Receiver<Box<dyn TsumugiFuture+ Send>>,
    receipt_channnel_sender:Sender<Box<dyn TsumugiFuture+ Send>>,
    global_tsumugi_controller:Option<&'static Box<TsumugiController>>,
    connect_tsumugi_controller:Vec<String>,
    global_connect_tsumugi_controller:Vec<Box<TsumugiController>>,
    tsumugi_controller_name:String,
    tsumugi_object_vector:Vec<Box<dyn TsumugiObject+ Send>>
}
trait TsumugiControllerTrait {
    fn new(tsumuginame:String)->TsumugiController;
    fn spown(&self,tsumuginame:String)->Box<TsumugiController> ;
    fn set_object(&mut self, tsumugi_object_list: Vec<Box<dyn TsumugiObject+ Send>>);
    fn run(& mut self, tsumugi_functions:& dyn Fn(& TsumugiController) -> Box<TsumugiController>);
}
impl  TsumugiControllerTrait  for TsumugiController {
    fn new(tsumuginame:String) -> TsumugiController  {
        let (mut txreceivebox, mut rxreceivebox): (Sender<Box<dyn TsumugiFuture+ Send>>, Receiver<Box<dyn TsumugiFuture+ Send>>) = mpsc::channel();
        let (txsendbox, rxsendbox): (Sender<Box<dyn TsumugiTypeChacher+ Send>>, Receiver<Box<dyn TsumugiTypeChacher+ Send>>) = mpsc::channel();
        let mut tumugi_receipt_list:Vec<Box<dyn TsumugiFuture+ Send>> = Vec::new();
        let mut pickup_list: Vec<Box<dyn TsumugiTypeChacher+ Send>> =  Vec::new();
        let mut tsumugi_connect_list: Vec<String> =  Vec::new();
        let mut tsumugi_object_list: Vec<Box<dyn TsumugiObject+ Send>> =  Vec::new();
        return TsumugiController {
            pickup_list: pickup_list,
            pickup_channnel_receiver: rxsendbox,
            pickup_channnel_sender: txsendbox,
            receipt_list: tumugi_receipt_list,
            receipt_channnel_receiver: rxreceivebox,
            receipt_channnel_sender: txreceivebox,
            global_tsumugi_controller: None,
            connect_tsumugi_controller: tsumugi_connect_list,
            global_connect_tsumugi_controller: vec![],
            tsumugi_controller_name: tsumuginame,
            tsumugi_object_vector: tsumugi_object_list };
    }

    fn spown(&self,tsumuginame:String) -> Box<TsumugiController> {
        let (mut txreceivebox, mut rxreceivebox): (Sender<Box<dyn TsumugiFuture+ Send>>, Receiver<Box<dyn TsumugiFuture+ Send>>) = mpsc::channel();
        let (txsendbox, rxsendbox): (Sender<Box<dyn TsumugiTypeChacher+ Send>>, Receiver<Box<dyn TsumugiTypeChacher+ Send>>) = mpsc::channel();
        let mut tumugi_receipt_list:Vec<Box<dyn TsumugiFuture+ Send>> = Vec::new();
        let mut pickup_list: Vec<Box<dyn TsumugiTypeChacher+ Send>> =  Vec::new();
        let mut tsumugi_connect_list: Vec<String> =  Vec::new();
        let mut tsumugi_object_list: Vec<Box<dyn TsumugiObject+ Send>> =  Vec::new();
        return Box::from(TsumugiController {
            pickup_list: pickup_list,
            pickup_channnel_receiver: rxsendbox,
            pickup_channnel_sender: txsendbox,
            receipt_list: tumugi_receipt_list,
            receipt_channnel_receiver: rxreceivebox,
            receipt_channnel_sender: txreceivebox,
            global_tsumugi_controller: self.global_tsumugi_controller,
            connect_tsumugi_controller: tsumugi_connect_list,
            global_connect_tsumugi_controller: vec![],
            tsumugi_controller_name: tsumuginame,
            tsumugi_object_vector: tsumugi_object_list
        });
    }

    fn set_object(&mut self, mut tsumugi_object_list: Vec<Box<dyn TsumugiObject+ Send>>) {
        self.tsumugi_object_vector.append(&mut tsumugi_object_list);
    }

    fn run(& mut self, tsumugi_function:  &dyn Fn(& TsumugiController) -> Box<TsumugiController>) {
        let mut tc_new = tsumugi_function(self);
        self.global_connect_tsumugi_controller.push(tc_new);
    }
}
pub fn spown_object_controller<'a>(tc:&'a TsumugiController) -> Box<TsumugiController> {
    let mut newtc = tc.spown("tsumugiobject".to_string());
    newtc.set_object(Vec::new());
    return newtc
}