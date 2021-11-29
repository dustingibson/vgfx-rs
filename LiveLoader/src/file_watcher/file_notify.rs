extern crate notify;

use crate::World;
use notify::{RecommendedWatcher, Watcher, RecursiveMode};
use std::sync::mpsc::channel;
use std::time::Duration;

pub struct FileNotify {
    base_folder: String
}

impl FileNotify {
    pub fn new(base_folder: String) -> FileNotify {
        return FileNotify {
            base_folder: base_folder
        }
    }

    pub fn watch_files(&self, world: &mut World)  -> notify::Result<()> {
        let (tx, rx) = channel();
        let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;
        watcher.watch(self.base_folder.to_string(), RecursiveMode::Recursive);
        loop {
            match rx.recv() {
                Ok(event) => {
                    match world.get_paths("areas".to_string()) {
                        Ok(paths) => { 
                            println!("{}", paths.join(",").to_string());
                            world.set_areas(paths);
                        },
                        Err(e) => {}
                    }
                    match world.get_paths("models".to_string()) {
                        Ok(paths) => { 
                            println!("{}", paths.join(",").to_string());
                            world.set_models(paths);
                        },
                        Err(e) => {}
                    }
                    world.save(self.base_folder.to_string());
                },
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    }
}