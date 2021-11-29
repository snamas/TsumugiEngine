use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use winapi::um::d3d12::ID3D12Device;
use tsumugi::controller::{TsumugiController, TsumugiControllerItemLifeTime, TsumugiControllerItemState, TsumugiControllerTrait};
use tsumugi::parcelreceptor_novalue::TsumugiParcelReceptorNoVal;
use tsumuStockCPU::TsumugiStockController;
use crate::gpu_store::{FigureDataLayer, TsumuGPUStoreData};
use crate::tg_device::TgID3D12Device;

///StoreListは3Dデータを管理する。Pathは3Dデータのパス
#[derive(Clone)]
pub struct TsumuGPUStoreList {
    pub(crate) list: Arc<Mutex<HashMap<&'static Path,FigureDataLayer>>>,
    pub(crate) tg_device :Arc<TgID3D12Device>
}

impl TsumuGPUStoreList {
    fn fetch_figuredata(&self,tc:TsumugiController){
        let thread_list = self.list.clone();
        let thread_device = self.tg_device.clone();
        let Object_antenna = TsumugiParcelReceptorNoVal::<TsumugiStockController>::new().subscribe_tc(Arc::new(move |parcel,tc| {
            let recept = *parcel.parcel.clone().unwrap();
            let recept = recept.clone();
            let thread_list = thread_list.clone();
            let thread_device = thread_device.clone();
            let antenna = TsumugiParcelReceptorNoVal::<&'static Path>::new().subscribe(Arc::new(move |parcel|{
                let parcel = *parcel.parcel.clone().unwrap();
                ///todo:ここ非同期じゃないので長時間ロックに注意
                thread_list.lock().unwrap()
                    .entry(parcel)
                    .or_insert(FigureDataLayer{ figure_data: None, material_layer: HashMap::new() }).figure_data
                    .get_or_insert(TsumuGPUStoreData::load(recept.0.lock().unwrap().get_mut(parcel).unwrap(),&thread_device));
                TsumugiControllerItemState::Fulfilled
            })).to_antenna().displayname("TsumugiReceptFigurePath").lifetime(TsumugiControllerItemLifeTime::Eternal);
            tc.tc.find("TsumugiStockCPU").unwrap().recept_channel_sender.send(antenna.into());
            TsumugiControllerItemState::Fulfilled
        })).to_antenna().displayname("check_store").lifetime(TsumugiControllerItemLifeTime::Once);
        tc.find("TsumugiStockCPU").unwrap().recept_channel_sender.send(Object_antenna.into());
    }
}