use crate::path::PathProvider;
use metadata_database::data::{Model, ModelBase, ModelType};
use std::collections::HashMap;
use std::path::PathBuf;

pub struct ComfyUiPathProvider {
    comfy_models_root: PathBuf,
    model_type_directory: Option<HashMap<ModelBase, String>>,
}

impl ComfyUiPathProvider {
    pub fn new(
        comfy_models_root: PathBuf,
        model_type_directory: Option<HashMap<ModelBase, String>>,
    ) -> Self {
        Self {
            comfy_models_root,
            model_type_directory,
        }
    }
}

impl PathProvider<Model> for ComfyUiPathProvider {
    fn get_path(&self, data: &Model) -> PathBuf {
        let mut dir =
            self.comfy_models_root
                .join(match data.model_type.unwrap_or(ModelType::Checkpoint) {
                    ModelType::Checkpoint => "checkpoints",
                    ModelType::Embedding => "embedding",
                    ModelType::Lora => "loras",
                    ModelType::Lycoris => "loras",
                    ModelType::ControlNet => "controlnet",
                    ModelType::Upscaler => "upscale_models",
                    ModelType::Vae => "vae",
                });
        if let Some(mtd) = &self.model_type_directory {
            if let Some(model_type) = data.base_model.and_then(|m| mtd.get(&m)) {
                dir = dir.join(model_type);
            }
        }
        dir.join(&data.file_name)
    }
}
