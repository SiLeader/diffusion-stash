use metadata_database::data::{GeneratedProduct, Model, ModelType};
use serde::{Deserialize, Serialize};
use server::path::{
    ComfyUiPathProvider, FlatFileNamePathProvider, FlatIdPathProvider, PathProvider,
};
use std::collections::HashMap;
use std::fs::File;
use std::sync::Arc;
use storage::Storage;
use storage::local::LocalStorage;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Config {
    pub server: ServerConfig,
    pub storage: StorageConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ServerConfig {
    pub listen: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct StorageConfig {
    #[serde(default)]
    pub backend: StorageBackend,
    pub model_path: PathProviderConfig,
    pub product_path: PathProviderConfig,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(tag = "provider")]
pub(crate) enum StorageBackend {
    #[default]
    Local,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "provider")]
pub(crate) enum PathProviderConfig {
    ComfyUI {
        root: String,
        #[serde(default, rename = "modelTypeDirectory")]
        model_type_directory: Option<HashMap<ModelType, String>>,
    },
    FileName {
        base: String,
    },
    Id {
        base: String,
    },
}

impl Config {
    pub(crate) fn from_yaml(path: &str) -> Config {
        let file = File::open(path).unwrap();
        serde_yml::from_reader(file).unwrap()
    }
}

impl PathProviderConfig {
    pub(crate) fn create_model_path_provider(&self) -> Arc<dyn PathProvider<Model>> {
        match self {
            PathProviderConfig::ComfyUI {
                root,
                model_type_directory,
            } => Arc::new(ComfyUiPathProvider::new(
                root.parse().unwrap(),
                model_type_directory.clone(),
            )),
            PathProviderConfig::FileName { base } => {
                Arc::new(FlatFileNamePathProvider::new(base.parse().unwrap()))
            }
            PathProviderConfig::Id { base } => {
                Arc::new(FlatIdPathProvider::new(base.parse().unwrap()))
            }
        }
    }

    pub(crate) fn create_product_path_provider(&self) -> Arc<dyn PathProvider<GeneratedProduct>> {
        match self {
            PathProviderConfig::ComfyUI { .. } => {
                panic!("ComfyUI is not a valid product path provider")
            }
            PathProviderConfig::FileName { base } => {
                Arc::new(FlatFileNamePathProvider::new(base.parse().unwrap()))
            }
            PathProviderConfig::Id { base } => {
                Arc::new(FlatIdPathProvider::new(base.parse().unwrap()))
            }
        }
    }
}

impl StorageBackend {
    pub(crate) fn create_storage(&self) -> Arc<dyn Storage + Send + Sync> {
        match self {
            StorageBackend::Local => Arc::new(LocalStorage),
        }
    }
}
