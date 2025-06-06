use crate::path::PathProvider;
use metadata_database::data::{GeneratedProduct, Model};
use std::path::PathBuf;

pub struct FlatIdPathProvider {
    base_directory: PathBuf,
}

impl FlatIdPathProvider {
    pub fn new(base_directory: PathBuf) -> Self {
        Self { base_directory }
    }
}

impl PathProvider<Model> for FlatIdPathProvider {
    fn get_path(&self, data: &Model) -> PathBuf {
        self.base_directory.join(data.id.to_string())
    }
}

impl PathProvider<GeneratedProduct> for FlatIdPathProvider {
    fn get_path(&self, data: &GeneratedProduct) -> PathBuf {
        self.base_directory.join(data.id.to_string())
    }
}
