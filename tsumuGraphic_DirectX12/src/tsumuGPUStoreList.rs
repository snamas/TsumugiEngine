use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use winapi::um::d3d12::{D3D12_RECT, D3D12_VIEWPORT, ID3D12Device};
use winapi::um::d3dcommon::D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST;
use tsumugi::controller::{TsumugiController, TsumugiControllerItemLifeTime, TsumugiControllerItemState, TsumugiControllerTrait};
use tsumugi::distributor::TsumugiParcelDistributor;
use tsumugi::parcelreceptor_novalue::TsumugiParcelReceptorNoVal;
use tsumuFigureStockCPU::TsumugiStockController;
use tsumugi::controller::TsumugiControllerItemLifeTime::Eternal;
use tsumugiShaderStock::TsumugiMaterial;
use crate::gpu_figure_store::{FigureDataLayer, MaterialLayer, TsumuGPUFigureDataStore};
use crate::material_loader::MaterialLoadDirectx12;
use crate::tg_device::TgID3D12Device;
use crate::tg_graphics_command_list::CpID3D12GraphicsCommandList;
use crate::TsumuGraphicObject;

///StoreListは3Dデータを管理する。Pathは3Dデータのパス
#[derive(Clone)]
pub struct TsumuGPUStoreList {
    pub list: Arc<Mutex<HashMap<&'static Path,FigureDataLayer>>>,
}

impl TsumuGraphicObject {
    pub fn fetch_figuredata(&self,tc:&TsumugiController){
        let thread_list = self.directx_store.list.clone();
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
        let thread_list = self.directx_store.list.clone();
        let thread_device = self.tg_device.clone();
        //todo:ここ読み取りするアンテナが一つなら出来るけど複数読み取りで正確にマテリアルが同期されない可能性があるよ
        let material_antenna = TsumugiParcelReceptorNoVal::<TsumugiMaterial>::new().subscribe(Arc::new(move|parcel|{
            let parcel = &parcel.parcel.clone().unwrap();
            let pipeline = parcel.load(&thread_device);
            thread_list.lock().unwrap()
                .entry(parcel.figure_path)
                .or_insert(FigureDataLayer{ figure_data: None, material_layer: HashMap::new() })
                .material_layer
                .insert(parcel.material_element_id,MaterialLayer{ material: pipeline, object_layer: Default::default() });
            TsumugiControllerItemState::Fulfilled
        })).to_antenna().displayname("TsumugiMaterial").parcelname("re_material").lifetime(Eternal);
        tc.find("TsumugiStockMaterials").unwrap().recept_channel_sender.send(material_antenna.into());
    }

}
impl TsumuGPUStoreList {
    pub fn debug_GPUStore(&self,tc:&TsumugiController){
        let thread_list = self.list.clone();
        let gpu_store_dist = TsumugiParcelDistributor::new(thread_list).displayname("gpu_store").lifetime(TsumugiControllerItemLifeTime::Eternal);
        tc.local_channel_sender.pickup_channel_sender.send(gpu_store_dist.into());
    }
    pub fn draw_figures(&self,tg_command_list:&mut [CpID3D12GraphicsCommandList]){
        let viewport = D3D12_VIEWPORT {
            TopLeftX: 0.0,
            TopLeftY: 0.0,
            Width: 1280 as f32,
            Height: 720 as f32,
            MinDepth: 0.0,
            MaxDepth: 1.0,
        };
        let scissorRect = D3D12_RECT {
            left: 0,
            top: 0,
            right: 1280 as i32,
            bottom: 720 as i32,
        };
        for mut figuredata in self.list.lock().unwrap().values_mut(){
            if let Some(figure) = &figuredata.figure_data{
                for storedata in figure{
                    //todo:ここ雑にマテリアル配列０番目を参照してるよ
                    tg_command_list[0].cp_set_pipeline_states(&mut figuredata.material_layer.get_mut(&0).unwrap().material[0].0);
                    tg_command_list[0].cp_set_graphics_root_signature(&mut figuredata.material_layer.get_mut(&0).unwrap().material[0].1);
                    tg_command_list[0].cp_iaset_primitive_topology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
                    tg_command_list[0].cp_iaset_vertex_buffers(0, &vec![storedata.vertex_view]);
                    tg_command_list[0].cp_iaset_index_buffer(&storedata.index_view);
                    tg_command_list[0].cp_rs_set_viewports(&vec![viewport]);
                    tg_command_list[0].cp_rs_set_scissor_rects(&vec![scissorRect]);
                    tg_command_list[0].cp_draw_indexed_instanced(storedata.index_len, 1, 0, 0, 0);
                }
            }
        }
    }
}