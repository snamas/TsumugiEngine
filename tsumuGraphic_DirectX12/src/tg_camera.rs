use image::EncodableLayout;
use tsumuObject::camera::Camera;
use crate::gpu_figure_store::MaterialCBV;

pub(crate) trait tsumugraphic_cameratrait{
    fn update_camera_resource(&self,camera_resource: &mut MaterialCBV);
}

impl tsumugraphic_cameratrait for Camera{

    fn update_camera_resource(&self, camera_resource: &mut MaterialCBV) {
        //ここで転置しているっぽかったのは、配列のレイアウトを列優先から行優先にするためだったけど、見た目列優先でもGPU上では行優先になってたよ
        let view_transpose = self.world_to_view();
        let proj_transpose = self.view_to_projection();
        let viewbuffer = view_transpose.data.as_bytes();
        let projbuffer = proj_transpose.data.as_bytes();
        let concat = [viewbuffer,projbuffer].concat();
        let buffer = concat.as_slice();
        camera_resource.0.mapvalue.as_mut().unwrap().copy_from_slice(buffer);
    }
}