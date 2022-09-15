mod file_watcher;
mod data;

//use data::model::Model;
use data::world::World;
use data::model::Model;
use data::model::AreaInstance;
use data::model::Face;
use data::model::TextureInfo;
use file_watcher::file_notify::FileNotify;


fn main() {
    let base_folder = "D:\\data\\vgfx2";
    let out_folder = "D:\\code\\vgfx-rs\\vgfx\\res";
    let file_notify = FileNotify::new(base_folder.to_string(), out_folder.to_string());
    if let Err(e) = file_notify.watch_files(base_folder.to_string()) {
        println!("error: {:?}", e)
    }
}