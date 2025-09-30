use metadata_database::data::{GeneratedProduct, Model, ModelBase};
use serde::{Deserialize, Serialize};
use server::path::{
    ComfyUiPathProvider, FlatFileNamePathProvider, FlatIdPathProvider, PathProvider,
};
use std::collections::HashMap;
use std::sync::Arc;
use storage::Storage;
use storage::local::LocalStorage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Config {
    pub server: ServerConfig,
    pub storage: StorageConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ServerConfig {
    pub listen: String,
    #[serde(default)]
    pub cors: CorsConfig,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct CorsConfig {
    #[serde(default)]
    pub allow_origins: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
        #[serde(default, rename = "directory")]
        directory: Option<HashMap<ModelType, String>>,
    },
    FileName {
        base: String,
    },
    Id {
        base: String,
    },
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ModelType {
    StableDiffusion15,
    StableDiffusionXl,
    PonyDiffusion,
    Illustrious,
    StableDiffusion30,
    StableDiffusion35,
    #[serde(rename = "flux1d")]
    Flux1D,
    #[serde(rename = "flux1s")]
    Flux1S,
}

impl Config {
    pub(crate) fn from_toml(path: &str) -> Config {
        let content = std::fs::read_to_string(path).unwrap();
        toml::from_str(&content).expect("Failed to parse config")
    }
}

impl PathProviderConfig {
    pub(crate) fn create_model_path_provider(&self) -> Arc<dyn PathProvider<Model>> {
        match self {
            PathProviderConfig::ComfyUI { root, directory } => Arc::new(ComfyUiPathProvider::new(
                root.parse().unwrap(),
                directory.clone().map(|d| {
                    d.into_iter()
                        .map(|(t, d)| (ModelBase::from(t), d.clone()))
                        .collect()
                }),
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

impl From<ModelType> for ModelBase {
    fn from(value: ModelType) -> Self {
        match value {
            ModelType::StableDiffusion15 => ModelBase::StableDiffusion15,
            ModelType::StableDiffusionXl => ModelBase::StableDiffusionXl,
            ModelType::PonyDiffusion => ModelBase::PonyDiffusion,
            ModelType::Illustrious => ModelBase::Illustrious,
            ModelType::StableDiffusion30 => ModelBase::StableDiffusion30,
            ModelType::StableDiffusion35 => ModelBase::StableDiffusion35,
            ModelType::Flux1D => ModelBase::Flux1D,
            ModelType::Flux1S => ModelBase::Flux1S,
        }
    }
}
