use crate::path::PathProvider;
use metadata_database::data::{GeneratedProduct, Model};
use std::path::PathBuf;

pub struct FlatFileNamePathProvider {
    base_directory: PathBuf,
}

impl FlatFileNamePathProvider {
    pub fn new(base_directory: PathBuf) -> Self {
        Self { base_directory }
    }
}

impl PathProvider<Model> for FlatFileNamePathProvider {
    fn get_path(&self, data: &Model) -> PathBuf {
        self.base_directory.join(data.file_name.as_str())
    }
}

impl PathProvider<GeneratedProduct> for FlatFileNamePathProvider {
    fn get_path(&self, data: &GeneratedProduct) -> PathBuf {
        self.base_directory.join(data.name.as_str())
    }
}
