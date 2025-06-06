mod comfyui;
mod file_name;
mod id;

use std::path::PathBuf;

pub trait PathProvider<T>: Send + Sync {
    fn get_path(&self, data: &T) -> PathBuf;
}
