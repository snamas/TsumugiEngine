use std::ffi::CString;
use std::io::Error;
use tsumuObject::Tsumugi3DObject;
use tsumuStockCPU::Import;
use crate::tg_device::TgID3D12Device;
use nalgebra;
use winapi::shared::dxgiformat::DXGI_FORMAT_R32G32B32_FLOAT;
use winapi::um::d3d12::{D3D12_APPEND_ALIGNED_ELEMENT, D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA, D3D12_INPUT_ELEMENT_DESC, D3D_ROOT_SIGNATURE_VERSION_1_0};

struct TsumuGPUStoreElement{
    object:Tsumugi3DObject,
    data:Import,
}
struct TsumugiGPUStoreList{

}

#[repr(C)]
#[derive(Clone, Debug, Copy)]
struct pointOnly(nalgebra::Point3<f32>);

impl TsumuGPUStoreElement {
    fn load(&self,tg_id3d12device: &TgID3D12Device){
        let mut vertexes_pos = Vec::<[f32; 3]>::new();
        let mut vertexes_id = Vec::<u32>::new();
        for mesh in self.data.document.meshes() {
            for prim in mesh.primitives() {
                let reader = prim.reader(|x| Some(&self.data.buffers[x.index()]));
                if let Some(iter) = reader.read_positions() {
                    vertexes_pos.append(&mut iter.collect::<Vec<[f32; 3]>>());
                }

                if let Some(iter) = reader.read_indices() {
                    vertexes_id.append(&mut iter.into_u32().collect());
                }
            }
        }
        let mut vertex_resource = Vec::<pointOnly>::new();
        vertex_resource.shrink_to_fit();
        //todo:かだい
        // let mut CpVertResource = tg_id3d12device.cp_create_buffer_resource(0, &vertexes_pos).unwrap_or_else(|v| { panic!("last OS error: {:?}", Error::last_os_error()) });
        // let mut mapvertdata = CpVertResource.cp_map(0,None).unwrap();
        // mapvertdata.copy_from_slice(&vertexes_pos);
        // let mut CpIndexResource = tg_id3d12device.cp_create_index_resource(0, &vertexes_id).unwrap_or_else(|v| { panic!("last OS error: {:?}", Error::last_os_error()) });
        // let mut mapindexdata = CpIndexResource.cp_map(0,None).unwrap();
        // mapindexdata.copy_from_slice(&vertexes_id);
        let inputElementDesc = vec![
            D3D12_INPUT_ELEMENT_DESC {
                SemanticName: CString::new("POSITION").expect("CString::new failed").into_raw(),
                SemanticIndex: 0,
                Format: DXGI_FORMAT_R32G32B32_FLOAT,
                InputSlot: 0,
                AlignedByteOffset: D3D12_APPEND_ALIGNED_ELEMENT,
                InputSlotClass: D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
                InstanceDataStepRate: 0,
            }
        ].into_boxed_slice();
        // let cp_d3d12_root_signature_desc: CpD3D12_ROOT_SIGNATURE_DESC = Default::default();
        // let rootSigBlob = cp_d3d12_root_signature_desc.cp_d3d12serialize_root_signature(D3D_ROOT_SIGNATURE_VERSION_1_0).unwrap_or_else(|v| { panic!("last OS error: {:?}", Error::last_os_error()) });
        // let mut rootsignature = cp_id3d12device.cp_create_root_signature(0, &rootSigBlob).unwrap_or_else(|v| { panic!("last OS error: {:?}", Error::last_os_error()) });
        // let mut cpgraphicsPipelineStateDesc = CpD3D12_GRAPHICS_PIPELINE_STATE_DESC::create_d3d12_graphics_pipeline_state_desc(&vsBlob, &psBlob, &inputElementDesc, &mut rootsignature, None, None, None);
        // let pipelineState = cp_id3d12device.cp_create_graphics_pipeline_state(&cpgraphicsPipelineStateDesc).unwrap_or_else(|v| {
        //     println!("last OS error: {:?}", Error::last_os_error());
        //     panic!("last OS error: {:?}", v)
        // });
    }
}