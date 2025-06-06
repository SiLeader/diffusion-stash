mod comfyui;
mod file_name;
mod id;

pub use comfyui::*;
pub use file_name::*;
pub use id::*;

use std::path::PathBuf;

pub trait PathProvider<T>: Send + Sync {
    fn get_path(&self, data: &T) -> PathBuf;
}
