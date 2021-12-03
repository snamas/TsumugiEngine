use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use winapi::um::d3d12::ID3D12Device;
use tsumugi::controller::{TsumugiController, TsumugiControllerItemLifeTime, TsumugiControllerItemState, TsumugiControllerTrait};
use tsumugi::distributor::TsumugiParcelDistributor;
use tsumugi::parcelreceptor_novalue::TsumugiParcelReceptorNoVal;
use tsumuFigureStockCPU::TsumugiStockController;
use tsumugiShaderStock::TsumugiMaterial;
use crate::gpu_figure_store::{FigureDataLayer, TsumuGPUFigureDataStore};
use crate::tg_device::TgID3D12Device;
use crate::tg_graphics_command_list::CpID3D12GraphicsCommandList;

///StoreListは3Dデータを管理する。Pathは3Dデータのパス
#[derive(Clone)]
pub struct TsumuGPUStoreList {
    pub list: Arc<Mutex<HashMap<&'static Path,FigureDataLayer>>>,
    pub(crate) tg_device :Arc<TgID3D12Device>
}

impl TsumuGPUStoreList {
    pub fn fetch_figuredata(&self,tc:&TsumugiController){
        let thread_list = self.list.clone();
        let thread_device = self.tg_device.clone();
        //StockControllerをゲットする。
        let figure_antenna = TsumugiParcelReceptorNoVal::<TsumugiStockController>::new().subscribe_tc(Arc::new(move |parcel, tc| {
            let recept = *parcel.parcel.clone().unwrap();
            let recept = recept.clone();
            let thread_list = thread_list.clone();
            let thread_device = thread_device.clone();
            //ゲットしたStockControllerを使って周知(announce)させたオブジェクトを取り込むよ
            let antenna = TsumugiParcelReceptorNoVal::<&'static Path>::new().subscribe(Arc::new(move |parcel|{
                let parcel = *parcel.parcel.clone().unwrap();
                ///todo:ここ非同期じゃないので長時間ロックに注意
                thread_list.lock().unwrap()
                    .entry(parcel)
                    .or_insert(FigureDataLayer{ figure_data: None, material_layer: HashMap::new() }).figure_data
                    .get_or_insert(TsumuGPUFigureDataStore::load(recept.0.lock().unwrap().get_mut(parcel).unwrap(), &thread_device));
                TsumugiControllerItemState::Fulfilled
            })).to_antenna().displayname("TsumugiReceptFigurePath").lifetime(TsumugiControllerItemLifeTime::Eternal);
            tc.tc.find("TsumugiStockCPU").unwrap().recept_channel_sender.send(antenna.into());
            TsumugiControllerItemState::Fulfilled
        })).to_antenna().displayname("check_store").lifetime(TsumugiControllerItemLifeTime::Once);
        tc.find("TsumugiStockCPU").unwrap().recept_channel_sender.send(figure_antenna.into());
    }
    pub fn fetch_materialdata(&self,tc:&TsumugiController){
        let thread_list = self.list.clone();
        let thread_device = self.tg_device.clone();
        //todo:ここ読み取りするアンテナが一つなら出来るけど複数読み取りで正確にマテリアルが同期されない可能性があるよ
        let material_antenna = TsumugiParcelReceptorNoVal::<TsumugiMaterial>::new().subscribe(Arc::new(move|parcel|{

            TsumugiControllerItemState::Fulfilled
        }));

    }
    pub fn debug_GPUStore(&self,tc:&TsumugiController){
        let thread_list = self.list.clone();
        let gpu_store_dist = TsumugiParcelDistributor::new(thread_list).displayname("gpu_store").lifetime(TsumugiControllerItemLifeTime::Eternal);
        tc.local_channel_sender.pickup_channel_sender.send(gpu_store_dist.into());
    }
    pub fn draw_figures<const N:usize>(&self,tg_command_list:&mut Vec<CpID3D12GraphicsCommandList>){
        for figuredata in self.list.lock().unwrap().iter(){
            if let Some(figure) = &figuredata.1.figure_data{
                for storedata in figure{

                }
            }
        }
    }
}