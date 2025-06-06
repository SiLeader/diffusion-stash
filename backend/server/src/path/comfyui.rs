use crate::path::PathProvider;
use metadata_database::data::{Model, ModelCategory, ModelType};
use std::collections::HashMap;
use std::path::PathBuf;

pub struct ComfyUiPathProvider {
    comfy_models_root: PathBuf,
    model_type_directory: Option<HashMap<ModelType, String>>,
}

impl ComfyUiPathProvider {
    pub fn new(
        comfy_models_root: PathBuf,
        model_type_directory: Option<HashMap<ModelType, String>>,
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
                .join(match data.category.unwrap_or(ModelCategory::Checkpoint) {
                    ModelCategory::Checkpoint => "checkpoints",
                    ModelCategory::Embedding => "embedding",
                    ModelCategory::Lora => "loras",
                    ModelCategory::Lycoris => "loras",
                    ModelCategory::ControlNet => "controlnet",
                    ModelCategory::Upscaler => "upscale_models",
                    ModelCategory::Vae => "vae",
                });
        if let Some(mtd) = &self.model_type_directory {
            if let Some(model_type) = data.model_type.and_then(|m| mtd.get(&m)) {
                dir = dir.join(model_type);
            }
        }
        dir.join(&data.file_name)
    }
}
