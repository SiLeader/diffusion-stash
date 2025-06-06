use metadata_database::data::ModelType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Config {
    pub server: ServerConfig,
    pub storage: StorageConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ServerConfig {
    pub listen: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct StorageConfig {
    pub model_path: PathProviderConfig,
    pub product_path: PathProviderConfig,
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
    pub(crate) fn from_yaml(path: &Path) -> Config {
        let file = File::open(path).unwrap();
        serde_yml::from_reader(file).unwrap()
    }
}
