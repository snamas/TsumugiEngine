use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use image::EncodableLayout;
use nalgebra::{Translation, Translation3, UnitQuaternion, Vector3};
use winapi::shared::minwindef::UINT;
use winapi::um::d3d12::{D3D12_CPU_PAGE_PROPERTY_UNKNOWN, D3D12_HEAP_PROPERTIES, D3D12_HEAP_TYPE_UPLOAD, D3D12_MEMORY_POOL_UNKNOWN, D3D12_RECT, D3D12_ROOT_DESCRIPTOR, D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT, D3D12_SHADER_VISIBILITY_ALL, D3D12_VIEWPORT, D3D_ROOT_SIGNATURE_VERSION_1_0, ID3D12Device};
use winapi::um::d3dcommon::D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST;
use tsumugi::controller::{TsumugiPortal, TsumugiControllerItemLifeTime, TsumugiControllerItemState, TsumugiControllerTrait, TsumugiPortalPlaneLocal};
use tsumugi::distributor::TsumugiParcelDistributor;
use tsumugi::parcelreceptor_novalue::TsumugiParcelReceptorNoVal;
use tsumuFigureStockCPU::TsumugiStockController;
use tsumugi::controller::TsumugiControllerItemLifeTime::Eternal;
use tsumugiShaderStock::TsumugiMaterial;
use tsumugiSimpleHashMap::TsumuHashMap;
use tsumuObject::camera::Camera;
use crate::gpu_figure_store::{FigureDataLayer, MaterialCBV, MaterialLayer, TsumuGPUFigureDataStore};
use crate::material_loader::MaterialLoadDirectx12;
use crate::tg_camera::tsumugraphic_cameratrait;
use crate::tg_descriptor_controller::TgID3D12DescriptorHeapList;
use crate::tg_device::TgID3D12Device;
use crate::tg_directx::{CpID3D12PipelineState, CpID3D12RootSignature};
use crate::tg_graphics_command_list::CpID3D12GraphicsCommandList;
use crate::tg_root_signature::{TgD3d12RootParameters, TgD3d12RootSignatureDesc};
use crate::TsumuGraphicObject;

///StoreListは3Dデータを管理する。Pathは3Dデータのパス
#[derive(Clone)]
pub struct TsumuGPUStoreList {
    pub list: Arc<Mutex<HashMap<&'static Path,FigureDataLayer>>>,
    pub camera:Arc<Mutex<Camera>>
}

impl TsumuGraphicObject {
    ///モデルデータをTsumugiStockCPUから引っ張ってくる関数。モデルデータをTsumugiStockCPUに流すことで、自動でそのモデルを描画出来るようにする。
    pub fn fetch_figuredata(&self,tc:&TsumugiPortal){
        let thread_list = self.directx_store.list.clone();
        let thread_device = self.tg_device.clone();
        //StockControllerをゲットする。
        let figure_antenna = TsumugiParcelReceptorNoVal::<TsumugiStockController>::new().subscribe_with_portal(Arc::new(move |parcel, tc| {
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
                    .or_insert(FigureDataLayer{ figure_data: None, material_layer: None }).figure_data
                    .get_or_insert(TsumuGPUFigureDataStore::load(recept.0.lock().unwrap().get_mut(parcel).unwrap(), &thread_device));
                TsumugiControllerItemState::Fulfilled
            })).to_antenna().displayname("TsumugiReceptFigurePath").lifetime(TsumugiControllerItemLifeTime::Eternal);
            tc.tp.find("TsumugiStockCPU").unwrap().recept_channel_sender.send(antenna.into());
            TsumugiControllerItemState::Fulfilled
        })).to_antenna().displayname("check_store").lifetime(TsumugiControllerItemLifeTime::Once);
        tc.find("TsumugiStockCPU").unwrap().recept_channel_sender.send(figure_antenna.into());
    }
    //todo:この関数。tsumuObjectにあるみたいに向こうの関数から引っ張った方がいいよ～～～～
    ///マテリアルデータをTsumugiStockMaterialsから引っ張ってくる関数。マテリアルをTsumugiStockMaterialsに流すことで、自動でそのマテリアルを描画出来るようにする。
    pub fn fetch_materialdata(&self,tc:&TsumugiPortal,descriptor_heap:&mut TgID3D12DescriptorHeapList){
        let thread_list = self.directx_store.list.clone();
        let thread_device = self.tg_device.clone();
        let thread_queue = self.tg_queue.clone();
        let mut thread_descriptor_heap = descriptor_heap.clone();
        //todo:ここ読み取りするアンテナが一つなら出来るけど複数読み取りで正確にマテリアルが同期されない可能性があるよ
        let material_antenna = TsumugiParcelReceptorNoVal::<TsumugiMaterial>::new().subscribe(Arc::new(move|parcel|{
            let parcel = &parcel.parcel.clone().unwrap();
            //todo:ディスクリプタヒープを無駄に更新してしまう可能性から、マテリアルを新規にロードするか、すでにロードされていたら、アップデートにするかの選択をさせておきたい。ディスクリプタハンドルの更新だけですむからお得？
            let pipeline = parcel.load(&thread_device,&thread_queue,&mut thread_descriptor_heap.clone());
            thread_list.lock().unwrap()
                .entry(parcel.figure_path)
                .or_insert(FigureDataLayer{ figure_data: None, material_layer: Option::None })
                .material_layer.get_or_insert(MaterialLayer{ material: TsumuHashMap::new(), object_layer: Default::default() })
                .material.overwrite_insert(parcel.material_element_id,pipeline);

            TsumugiControllerItemState::Fulfilled
        })).to_antenna().displayname("TsumugiMaterial").parcelname("re_material").lifetime(Eternal);
        tc.find("TsumugiStockMaterials").unwrap().recept_channel_sender.send(material_antenna.into());
    }
    //todo:ここでは実行速度の低下を許容して、カメラを毎フレーム更新する。カメラの都度更新で大幅な速度上昇は得られないと考えたため。更新案が思いついたら直す。
    pub(crate) fn fetch_cameradata(&self,tc:&TsumugiPortal,descriptor_heap:&mut TgID3D12DescriptorHeapList) ->MaterialCBV{
        let descriptor_handle = descriptor_heap.cbv_srv_uav.allocate_dynamic_descriptor_handle().unwrap_or_else(||{panic!("cbv allocate Failed")});
        let heapProperties = D3D12_HEAP_PROPERTIES {
            Type: D3D12_HEAP_TYPE_UPLOAD,
            CPUPageProperty: D3D12_CPU_PAGE_PROPERTY_UNKNOWN,
            MemoryPoolPreference: D3D12_MEMORY_POOL_UNKNOWN,
            CreationNodeMask: 0,
            VisibleNodeMask: 0,
        };
        //todo:ここ一瞬カメラの位置が切り替わっているので、なんとかしたい
        let mut camera:Camera = Camera::new(Translation3::new(0f32,0f32,-3f32), UnitQuaternion::from_scaled_axis(Vector3::new(0f32, std::f32::consts::PI, 0f32)));
        //ここで転置しているのは、配列のレイアウトを列優先から行優先にするため
        let view_transpose = camera.world_to_view();
        let proj_transpose = camera.view_to_projection();
        let viewbuffer = view_transpose.data.as_bytes();
        let projbuffer = proj_transpose.data.as_bytes();
        let concat = [viewbuffer,projbuffer].concat();
        let buffer = concat.as_slice();
        let mut resource = self.tg_device.tg_create_constant_resource_from_slice(buffer, heapProperties, 0).unwrap().tg_slice_map(0, None, buffer).unwrap();
        resource.mapvalue.as_mut().unwrap().copy_from_slice(buffer);

        self.tg_device.tg_create_constant_buffer_view(&resource,&descriptor_handle);

        let mut camera_resource = MaterialCBV(resource,descriptor_handle);
        camera.update_camera_resource(&mut camera_resource);
        let camera_thread = self.directx_store.camera.clone();
        let func = move |arc_camera: &TsumugiParcelReceptorNoVal<Camera>, tpl: &TsumugiPortalPlaneLocal| {
            let camera = arc_camera.parcel.clone().unwrap();
            *camera_thread.lock().unwrap() = *camera;
            TsumugiControllerItemState::Fulfilled
        };
        let recept_object = TsumugiParcelReceptorNoVal::new().subscribe_with_portal(Arc::new(func)).to_antenna().lifetime(Eternal).displayname("camera_object");
        tc.find("tsumugi3dObject").unwrap().wait(recept_object.into());
        camera_resource
    }

}
impl TsumuGPUStoreList {
    pub fn debug_GPUStore(&self,tc:&TsumugiPortal){
        let thread_list = self.list.clone();
        let gpu_store_dist = TsumugiParcelDistributor::new(thread_list).displayname("gpu_store").lifetime(TsumugiControllerItemLifeTime::Eternal);
        tc.local_channel_sender.distributor_channel_sender.send(gpu_store_dist.into());
    }
    pub(crate) fn draw_figures(&self,tg_command_list:&mut [CpID3D12GraphicsCommandList],camera_material:&mut MaterialCBV){

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
                if let Some(material) =  &mut figuredata.material_layer{
                    figure.iter().enumerate().for_each(|(i,storedata)|{
                        if let Some(material) = material.material.get_mut(i){
                            tg_command_list[0].cp_set_pipeline_states(&mut material.0);
                            tg_command_list[0].cp_set_graphics_root_signature(&mut material.1);
                            tg_command_list[0].tg_set_graphics_root_constant_buffer_view(&camera_material.0);
                            for material_cbv in &material.2{
                                tg_command_list[0].tg_set_graphics_root_constant_buffer_view(&material_cbv.0);
                            }
                            for material_table in &material.3{
                                tg_command_list[0].tg_set_graphics_root_descriptor_table(&material_table.0, &material_table.1);
                            }
                        }else{
                            tg_command_list[0].cp_set_pipeline_states(&mut material.material.vector[0].as_mut().unwrap().0);
                            tg_command_list[0].cp_set_graphics_root_signature(&mut material.material.vector[0].as_mut().unwrap().1);
                            tg_command_list[0].tg_set_graphics_root_constant_buffer_view(&camera_material.0);
                            for material_cbv in &material.material.vector[0].as_mut().unwrap().2{
                                tg_command_list[0].tg_set_graphics_root_constant_buffer_view(&material_cbv.0);
                            }
                            for material_table in &material.material.vector[0].as_mut().unwrap().3{
                                tg_command_list[0].tg_set_graphics_root_descriptor_table(&material_table.0, &material_table.1);
                            }
                        }
                        tg_command_list[0].cp_iaset_primitive_topology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
                        tg_command_list[0].cp_iaset_vertex_buffers(0, &vec![storedata.vertex_view]);
                        tg_command_list[0].cp_iaset_index_buffer(&storedata.index_view);
                        tg_command_list[0].cp_rs_set_viewports(&vec![viewport]);
                        tg_command_list[0].cp_rs_set_scissor_rects(&vec![scissorRect]);
                        tg_command_list[0].cp_draw_indexed_instanced(storedata.index_len, 1, 0, 0, 0);
                    });
                }
            }
        }
    }
}