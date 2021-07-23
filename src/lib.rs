use std::any::{TypeId, Any};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::task::Poll;
use std::option::Option::Some;

trait TsumugiFuture:TsumugiTypeChacher{
    fn poll(self: &mut Self) -> Poll<()>;
    fn inputItem(self: &mut Self,inputItem: Box<dyn TsumugiTypeChacher+ Send>);
}
trait TsumugiTypeChacher{
    fn as_any(&mut self) -> &mut dyn Any;
    fn typehash(&self)->TypeId;
}
trait TsumugiObject{
    fn onCreate(&self);
}
pub struct TsumugiController<'a> {
    pickup_list:Vec<Box<dyn TsumugiTypeChacher+ Send>>,
    pickup_channnel_receiver:Receiver<Box<dyn TsumugiTypeChacher+ Send>>,
    pickup_channnel_sender:Sender<Box<dyn TsumugiTypeChacher+ Send>>,
    receipt_list:Vec<Box<dyn TsumugiFuture+ Send>>,
    receipt_channnel_receiver:Receiver<Box<dyn TsumugiFuture+ Send>>,
    receipt_channnel_sender:Sender<Box<dyn TsumugiFuture+ Send>>,
    global_tsumugi_controller:Option<&'a TsumugiController<'a>>,
    connect_tsumugi_controller:Vec<String>,
    global_connect_tsumugi_controller:Vec<String>,
    tsumugi_controller_name:String,
    tsumugi_object_vector:Vec<Box<dyn TsumugiObject+ Send>>
}
trait TsumugiControllerTrait<'a> {
    fn new(tsumuginame:String)->TsumugiController<'a>;
    fn spown(&self,tsumuginame:String)->TsumugiController<'a> ;
    fn OnCreate(&mut self,tsumugi_object_list: Vec<Box<dyn TsumugiObject+ Send>>);
    fn Run(&self,tsumugi_functions:Vec<Box<dyn Fn(&'a TsumugiController)->TsumugiController<'a>>>){
        for tsumugi_function in tsumugi_functions{
        }
    }
}
impl<'a>  TsumugiControllerTrait<'a>  for TsumugiController<'a> {
    fn new(tsumuginame:String) -> TsumugiController<'a>  {
        let (mut txreceivebox, mut rxreceivebox): (Sender<Box<dyn TsumugiFuture+ Send>>, Receiver<Box<dyn TsumugiFuture+ Send>>) = mpsc::channel();
        let (txsendbox, rxsendbox): (Sender<Box<dyn TsumugiTypeChacher+ Send>>, Receiver<Box<dyn TsumugiTypeChacher+ Send>>) = mpsc::channel();
        let mut tumugiReceiptList:Vec<Box<dyn TsumugiFuture+ Send>> = Vec::new();
        let mut pickup_list: Vec<Box<dyn TsumugiTypeChacher+ Send>> =  Vec::new();
        let mut tsumugi_connect_list: Vec<String> =  Vec::new();
        let mut tsumugi_object_list: Vec<Box<dyn TsumugiObject+ Send>> =  Vec::new();
        return TsumugiController {
            pickup_list: pickup_list,
            pickup_channnel_receiver: rxsendbox,
            pickup_channnel_sender: txsendbox,
            receipt_list: tumugiReceiptList,
            receipt_channnel_receiver: rxreceivebox,
            receipt_channnel_sender: txreceivebox,
            global_tsumugi_controller: None,
            connect_tsumugi_controller: tsumugi_connect_list,
            global_connect_tsumugi_controller: vec![],
            tsumugi_controller_name: tsumuginame,
            tsumugi_object_vector: tsumugi_object_list };
    }

    fn spown(&self,tsumuginame:String) -> TsumugiController<'a>  {
        let (mut txreceivebox, mut rxreceivebox): (Sender<Box<dyn TsumugiFuture+ Send>>, Receiver<Box<dyn TsumugiFuture+ Send>>) = mpsc::channel();
        let (txsendbox, rxsendbox): (Sender<Box<dyn TsumugiTypeChacher+ Send>>, Receiver<Box<dyn TsumugiTypeChacher+ Send>>) = mpsc::channel();
        let mut tumugiReceiptList:Vec<Box<dyn TsumugiFuture+ Send>> = Vec::new();
        let mut pickup_list: Vec<Box<dyn TsumugiTypeChacher+ Send>> =  Vec::new();
        let mut tsumugi_connect_list: Vec<String> =  Vec::new();
        let mut tsumugi_object_list: Vec<Box<dyn TsumugiObject+ Send>> =  Vec::new();
        return TsumugiController {
            pickup_list: pickup_list,
            pickup_channnel_receiver: rxsendbox,
            pickup_channnel_sender: txsendbox,
            receipt_list: tumugiReceiptList,
            receipt_channnel_receiver: rxreceivebox,
            receipt_channnel_sender: txreceivebox,
            global_tsumugi_controller: self.global_tsumugi_controller,
            connect_tsumugi_controller: tsumugi_connect_list,
            global_connect_tsumugi_controller: vec![],
            tsumugi_controller_name: tsumuginame,
            tsumugi_object_vector: tsumugi_object_list };
    }

    fn OnCreate(&mut self, mut tsumugi_object_list: Vec<Box<dyn TsumugiObject+ Send>>) {
        self.tsumugi_object_vector.append(&mut tsumugi_object_list);
    }
}
pub fn spown_object_controller<'a>(tc:&'a TsumugiController<'a>) ->TsumugiController<'a> {
    let mut newtc = tc.spown("tsumugiobject".to_string());
    newtc.OnCreate(Vec::new());
    return newtc
}