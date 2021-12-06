use std::any::Any;
use std::collections::HashMap;
use std::io::Read;
use std::path::Path;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;
use imgui::{Condition, Ui};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use tsumugi::controller::{DebugKit, TsumugiController, TsumugiController_threadlocal, TsumugiControllerItemLifeTime, TsumugiControllerItemState, TsumugiControllerTrait, TsumugiObject};
use imgui_sdl2_support;
use imgui_glow_renderer;
use glow;
use glow::HasContext;
use imgui_glow_renderer::AutoRenderer;
use imgui_sdl2_support::SdlPlatform;
use sdl2::hint::Hint::Default;
use sdl2::video::{GLProfile, Window};
use tsumugi::parcelreceptor_novalue::TsumugiParcelReceptorNoVal;
use tsumuObject::{Tsumugi3DObject, TsumugiObjectController};
use tsumuFigureStockCPU::TsumugiStockController;
use tsumugiShaderStock::TsumugiShaderStockController;
use tsumuGraphic_DirectX12::gpu_figure_store::FigureDataLayer;


static TSUMUGI_DEBUG_WINDOWS: &str = "TsumugiDebugWin";
#[derive(Clone)]
struct TsumguiWindow {
    objectlist: Arc<RwLock<Arc<Mutex<HashMap<u64, Tsumugi3DObject>>>>>,
    cpu_store_list: Arc<RwLock<Box<TsumugiStockController>>>,
    shader_list:Arc<RwLock<TsumugiShaderStockController>>,
    gpu_store_list:Arc<RwLock<Arc<Mutex<HashMap<&'static Path,FigureDataLayer>>>>>,
    debug_list:Arc<Mutex<HashMap<String,DebugKit>>>
}

fn glow_context(window: &Window) -> glow::Context {
    unsafe {
        glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _)
    }
}

impl TsumguiWindow {
    fn check_object_list(&self, tc: &TsumugiController_threadlocal) {
        let mut thread_arc = self.objectlist.clone();
        let Object_antenna = TsumugiParcelReceptorNoVal::<TsumugiObjectController>::new().subscribe(Arc::new(move |parcel| {
            let recept = parcel.parcel.as_ref().unwrap().object_hashmap.clone();
            *thread_arc.write().unwrap() = recept;
            TsumugiControllerItemState::Fulfilled
        })).to_antenna().displayname("check_object").lifetime(TsumugiControllerItemLifeTime::Once);
        tc.tc.find("tsumugi3dObject").unwrap().recept_channel_sender.send(Object_antenna.into());
    }
    fn check_store_list(&self, tc: &TsumugiController_threadlocal) {
        let mut thread_arc = self.cpu_store_list.clone();
        let Object_antenna = TsumugiParcelReceptorNoVal::<TsumugiStockController>::new().subscribe(Arc::new(move |parcel| {
            let recept = parcel.parcel.clone().unwrap();
            *thread_arc.write().unwrap() = recept;
            TsumugiControllerItemState::Fulfilled
        })).to_antenna().displayname("check_store").lifetime(TsumugiControllerItemLifeTime::Once);
        tc.tc.find("TsumugiStockCPU").unwrap().recept_channel_sender.send(Object_antenna.into());
    }
    fn check_shader_list(&self, tc: &TsumugiController_threadlocal) {
        let mut thread_arc = self.shader_list.clone();
        let object_antenna = TsumugiParcelReceptorNoVal::<TsumugiShaderStockController>::new().subscribe(Arc::new(move |parcel| {
            let recept = parcel.parcel.clone().unwrap();
            *thread_arc.write().unwrap() = *recept;
            TsumugiControllerItemState::Fulfilled
        })).to_antenna().displayname("check_shader_store").lifetime(TsumugiControllerItemLifeTime::Once);
        tc.tc.find("TsumugiStockMaterials").unwrap().recept_channel_sender.send(object_antenna.into());
    }
    fn check_gpu_store_list(&self, tc: &TsumugiController_threadlocal) {
        let mut thread_arc = self.gpu_store_list.clone();
        let Object_antenna = TsumugiParcelReceptorNoVal::<Arc<Mutex<HashMap<&'static Path,FigureDataLayer>>>>::new().subscribe(Arc::new(move |parcel| {
            let recept = *parcel.parcel.clone().unwrap();
            *thread_arc.write().unwrap() = recept;
            TsumugiControllerItemState::Fulfilled
        })).to_antenna().displayname("check_gpu_store").lifetime(TsumugiControllerItemLifeTime::Once);
        tc.tc.find("tsumuGraphicDx12").unwrap().recept_channel_sender.send(Object_antenna.into());
    }
    fn check_debug_list(&self, tc: &TsumugiController_threadlocal){
        let mut thread_arc = self.debug_list.clone();
        let antenna = TsumugiParcelReceptorNoVal::<DebugKit>::new().subscribe(Arc::new(move |parcel|{
            let plane_name = parcel.parcel.clone().unwrap().plane_name;
            let debug = parcel.parcel.clone().unwrap();
            thread_arc.lock().unwrap().insert(plane_name, *debug.clone());
            TsumugiControllerItemState::Fulfilled
        })).to_antenna().lifetime(TsumugiControllerItemLifeTime::Eternal);
        tc.tc.local_channel_sender.recept_channel_sender.send(antenna.into());
    }
}

impl TsumugiObject for TsumguiWindow {
    fn on_create(&self, tc: &TsumugiController_threadlocal) {
        self.check_object_list(tc);
        self.check_store_list(tc);
        self.check_shader_list(tc);
        self.check_gpu_store_list(tc);
        self.check_debug_list(tc);
        let thread_objectlist = self.objectlist.clone();
        let thread_cpu_store_list = self.cpu_store_list.clone();
        let thread_shader_store_list = self.shader_list.clone();
        let thread_gpu_store_list = self.gpu_store_list.clone();
        let thread_debug_list = self.debug_list.clone();
        thread::spawn(move || {
            let sdl = sdl2::init().unwrap();
            let video_subsystem = sdl.video().unwrap();
            let gl_attr = video_subsystem.gl_attr();
            gl_attr.set_context_version(3, 3);
            gl_attr.set_context_profile(GLProfile::Core);

            let window = video_subsystem
                .window("Hello imgui-rs!", 1280, 720)
                .allow_highdpi()
                .opengl()
                .position_centered()
                .resizable()
                .build()
                .unwrap();

            let gl_context = window.gl_create_context().unwrap();
            window.gl_make_current(&gl_context).unwrap();
            window.subsystem().gl_set_swap_interval(1).unwrap();

            let gl = glow_context(&window);

            let mut imgui = imgui::Context::create();
            imgui.set_ini_filename(None);
            imgui.set_log_filename(None);

            imgui
                .fonts()
                .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

            let mut platform = SdlPlatform::init(&mut imgui);
            let mut renderer = AutoRenderer::initialize(gl, &mut imgui).unwrap();
            let mut event_pump = sdl.event_pump().unwrap();

            'main: loop {
                for event in event_pump.poll_iter() {
                    platform.handle_event(&mut imgui, &event);

                    if let Event::Quit { .. } = event {
                        break 'main;
                    }
                }
                platform.prepare_frame(&mut imgui, &window, &event_pump);

                let ui = imgui.new_frame();
                if let Some(_t) = ui.tree_node("ObjectList") {
                    ui.child_window("ObjectList")
                        .size([0.0, 60.0])
                        .border(true)
                        .build(|| {
                            for object in thread_objectlist.read().unwrap().lock().unwrap().iter() {
                                ui.text(format!("Object Name:{}", object.1.name));
                            }
                        });
                }
                if let Some(_t) = ui.tree_node("CPUStoreList") {
                    ui.child_window("CPUStoreList")
                        .size([0.0, 60.0])
                        .border(true)
                        .build(|| {
                            for object in thread_cpu_store_list.read().unwrap().0.lock().unwrap().keys() {
                                ui.text(format!("Object Name:{}", object.to_str().unwrap()));
                            }
                        });
                }
                if let Some(_t) = ui.tree_node("MaterialStoreList") {
                    ui.child_window("MaterialStoreList")
                        .size([0.0, 60.0])
                        .border(true)
                        .build(|| {
                            for object in thread_shader_store_list.read().unwrap().0.lock().unwrap().keys() {
                                ui.text(format!("Object Name:{}", object));
                            }
                        });
                }
                if let Some(_t) = ui.tree_node("GPUStoreList") {
                    ui.child_window("GPUStoreList")
                        .size([0.0, 60.0])
                        .border(true)
                        .build(|| {
                            for object in thread_gpu_store_list.read().unwrap().lock().unwrap().iter() {
                                ui.text(format!("Object Name:{}", object.0.to_str().unwrap()));
                            }
                        });
                }
                if let Some(_t) = ui.tree_node("Debug") {
                    for debugkit in thread_debug_list.lock().unwrap().iter(){
                        if let Some(_t) = ui.tree_node(debugkit.0){
                            if let Some(_t) = ui.tree_node("recept"){
                                ui.child_window(debugkit.0)
                                    .size([0.0, 60.0])
                                    .border(true)
                                    .build(|| {
                                        for object in &debugkit.1.parcel.recept_list {
                                            ui.text(format!("TypeID:{:?},Name:{}", object.0,object.1));
                                        }
                                    });
                            }
                            if let Some(_t) = ui.tree_node("pickup"){
                                ui.child_window(debugkit.0)
                                    .size([0.0, 60.0])
                                    .border(true)
                                    .build(|| {
                                        for object in &debugkit.1.parcel.pickup_list {
                                            ui.text(format!("TypeID:{:?},Name:{}", object.0,object.1));
                                        }
                                    });
                            }
                        }
                    }
                }
                ui.show_demo_window(&mut true);
                let draw_data = imgui.render();
                unsafe { renderer.gl_context().clear(glow::COLOR_BUFFER_BIT) };
                renderer.render(draw_data).unwrap();
                window.gl_swap_window();
            }
        });
    }
}


pub fn spown_debug_window_handler(tc: &Box<TsumugiController>) -> Box<TsumugiController> {
    let mut newtc = tc.spown(TSUMUGI_DEBUG_WINDOWS.to_string());
    newtc.set_objects(vec![
        Box::new(TsumguiWindow {
            objectlist: Arc::new(RwLock::new(Arc::new(Mutex::new(HashMap::default())))),
            cpu_store_list: Arc::new(RwLock::new(Box::new(TsumugiStockController{ 0: Arc::new(Mutex::new(HashMap::default())) }))),
            shader_list: Arc::new(RwLock::new(TsumugiShaderStockController::default())),
            gpu_store_list: Arc::new(RwLock::new(Arc::new(Mutex::new(HashMap::default())))),
            debug_list: Arc::new(Mutex::new(HashMap::new()))
        }),
    ]);
    return newtc;
}
