use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::sync::{Arc, Mutex};
type Buffer = Vec<u8>;
struct ShaderStore{
    pub list: Arc<Mutex<HashMap<&'static Path,Buffer>>>,
}

impl ShaderStore {
    fn load(&self,shader_path:&'static Path){
        let mut f = File::open(shader_path).unwrap();
        let mut buff = Vec::new();
        //todo:適当に最後まで読んでいるので、非同期ではないよ。
        let buffersize = f.read_to_end(&mut buff).unwrap();
        self.list.lock().unwrap().entry(shader_path).or_insert(buff);
    }
}