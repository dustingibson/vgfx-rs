mod file_watcher;
mod data;

//use data::model::Model;
use data::world::World;
use data::model::Model;
use data::model::ModelInstance;
use data::model::AreaInstance;
use file_watcher::file_notify::FileNotify;


fn main() {
    let base_folder = "C:\\data\\vgfx";
    let out_folder = "C:\\code\\vgfx-rs\\vgfx\\res";
    let mut world = World::new(base_folder.to_string());
    let file_notify = FileNotify::new(base_folder.to_string(),  out_folder.to_string());
    if let Err(e) = file_notify.watch_files(&mut world) {
        println!("error: {:?}", e)
    }
}