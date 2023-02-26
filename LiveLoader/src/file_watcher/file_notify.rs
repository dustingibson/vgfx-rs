extern crate notify;

use crate::World;
use notify::{RecommendedWatcher, Watcher, RecursiveMode};
use std::sync::mpsc::channel;
use std::time::Duration;

pub struct FileNotify {
    base_folder: String,
    out_folder: String
}

impl FileNotify {
    pub fn new(base_folder: String, out_folder: String) -> FileNotify {
        return FileNotify {
            base_folder: base_folder,
            out_folder: out_folder
        }
    }

    pub fn watch_files(&self, base_folder: String)  -> notify::Result<()> {
        let (tx, rx) = channel();
        let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;
        watcher.watch(self.base_folder.to_string(), RecursiveMode::Recursive);
        let accept_areas_paths = vec![".obj".to_string(), ".json".to_string(), ".mtl".to_string()];
        let accept_png_paths = vec![".png".to_string()];
        loop {
            match rx.recv() {
                Ok(event) => {
                    let mut world = World::new(base_folder.to_string());
                    match world.get_paths("areas".to_string(), &accept_areas_paths) {
                        Ok(paths) => { 
                            println!("Areas: {}", paths.join(",").to_string());
                            world.set_areas(paths);
                        },
                        Err(e) => {}
                    }
                    match world.get_dir("models".to_string()) {
                        Ok(dirs) => {
                            for dir in dirs {
                                match world.get_paths(dir.to_string(), &accept_areas_paths) {
                                    Ok(paths) => { 
                                        println!("Models: {}", paths.join(",").to_string());
                                        world.set_models(paths);
                                    },
                                    Err(_e) => {
                                        panic!("{}", _e)
                                    }
                                }
                            }
                        },
                        Err(_e) => {
                            panic!("{}", _e);
                        }
                    }
                    match world.get_paths("images".to_string(), &accept_png_paths) {
                        Ok(paths) => {
                            println!("Images {}", paths.join(",").to_string());
                            world.set_image(paths);
                        },
                        Err(_e) => {
                            panic!("{}", _e)
                        }
                    }
                    world.save(self.out_folder.to_string());
                },
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    }
}