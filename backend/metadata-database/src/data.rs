use crate::entity::sea_orm_active_enums::{ModelBaseEnum, ModelTypeEnum};
use crate::entity::{ai_model, generated_product};
use sea_orm::prelude::{DateTimeUtc, Uuid};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

macro_rules! uuid_id {
    ($name:ident) => {
        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub struct $name(Uuid);

        impl From<Uuid> for $name {
            fn from(uuid: Uuid) -> Self {
                Self(uuid)
            }
        }

        impl From<$name> for Uuid {
            fn from(uuid: $name) -> Self {
                uuid.0
            }
        }

        impl $name {
            pub fn new() -> Self {
                Self(Uuid::new_v4())
            }

            pub fn into_inner(self) -> Uuid {
                self.0
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl TryFrom<String> for $name {
            type Error = uuid::Error;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Uuid::parse_str(&value).map(Self)
            }
        }
    };
}

pub type ModelBase = ModelBaseEnum;
pub type ModelType = ModelTypeEnum;

uuid_id!(ModelId);
uuid_id!(GeneratedProductId);

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    pub id: ModelId,
    pub file_name: String,
    pub name: String,
    pub description: String,
    pub base_model: Option<ModelBase>,
    #[serde(rename = "type")]
    pub model_type: Option<ModelType>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneratedProduct {
    pub id: GeneratedProductId,
    pub name: String,
    pub models: Vec<Model>,
    pub mime_type: String,
    pub positive_prompt: Option<String>,
    pub negative_prompt: Option<String>,
    pub sampler_name: Option<String>,
    pub scheduler_name: Option<String>,
    pub step_count: Option<i32>,
    pub cfg_scale: Option<f32>,
    pub seed: Option<i64>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

impl From<Model> for ai_model::Model {
    fn from(value: Model) -> Self {
        Self {
            id: value.id.into(),
            name: value.name,
            description: value.description,
            file_name: value.file_name,
            base_model: value.base_model,
            model_type: value.model_type,
            created_at: value.created_at.naive_utc(),
            updated_at: value.updated_at.naive_utc(),
        }
    }
}

impl From<ai_model::Model> for Model {
    fn from(value: ai_model::Model) -> Self {
        Self {
            id: value.id.into(),
            name: value.name,
            description: value.description,
            file_name: value.file_name,
            base_model: value.base_model,
            model_type: value.model_type,
            created_at: value.created_at.and_utc(),
            updated_at: value.updated_at.and_utc(),
        }
    }
}

type Associated = (generated_product::Model, Vec<ai_model::Model>);

impl From<GeneratedProduct> for Associated {
    fn from(value: GeneratedProduct) -> Self {
        let models = value.models.into_iter().map(|model| model.into()).collect();
        (
            generated_product::Model {
                id: value.id.into_inner(),
                name: value.name,
                mime_type: value.mime_type,
                positive_prompt: value.positive_prompt,
                negative_prompt: value.negative_prompt,
                sampler_name: value.sampler_name,
                scheduler_name: value.scheduler_name,
                step_count: value.step_count,
                cfg_scale: value.cfg_scale,
                seed: value.seed,
                created_at: value.created_at.naive_utc(),
                updated_at: value.updated_at.naive_utc(),
            },
            models,
        )
    }
}

impl From<Associated> for GeneratedProduct {
    fn from(value: Associated) -> Self {
        let (product, models) = value;
        Self {
            id: product.id.into(),
            name: product.name,
            models: models.into_iter().map(|m| m.into()).collect(),
            mime_type: product.mime_type,
            positive_prompt: product.positive_prompt,
            negative_prompt: product.negative_prompt,
            sampler_name: product.sampler_name,
            scheduler_name: product.scheduler_name,
            step_count: product.step_count,
            cfg_scale: product.cfg_scale,
            seed: product.seed,
            created_at: product.created_at.and_utc(),
            updated_at: product.updated_at.and_utc(),
        }
    }
}
