use crate::endpoints::IntoNullIfEmpty;
use crate::endpoints::error_response::ErrorResponse;
use crate::{DataStorage, read_file, try_db, try_storage};
use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_web::web::Data;
use actix_web::{HttpResponse, post};
use chrono::Utc;
use log::error;
use metadata_database::MetadataDatabase;
use metadata_database::data::{GeneratedProduct, GeneratedProductId, ModelId};
use serde::Serialize;
use std::io::Read;

#[derive(MultipartForm)]
pub(super) struct AddProductRequest {
    file: TempFile,
    name: Text<String>,
    models: Vec<Text<String>>,
    positive_prompt: Option<Text<String>>,
    negative_prompt: Option<Text<String>>,
    sampler_name: Option<Text<String>>,
    scheduler_name: Option<Text<String>>,
    step_count: Option<Text<String>>,
    cfg_scale: Option<Text<String>>,
    seed: Option<Text<String>>,
}

#[derive(Serialize)]
struct AddProductResponse {
    product: GeneratedProduct,
}

#[post("/v1/products")]
pub(super) async fn handle_add_product(
    request: MultipartForm<AddProductRequest>,
    metadata: Data<MetadataDatabase>,
    storage: Data<DataStorage>,
) -> HttpResponse {
    let request = request.into_inner();
    let models = {
        let mut models = Vec::new();
        for model in request.models {
            let model_id = match ModelId::try_from(model.into_inner()) {
                Ok(id) => id,
                Err(e) => {
                    error!("Invalid model id: {}", e);
                    return ErrorResponse::InvalidModelIdFormat.into();
                }
            };
            let Some(model) = try_db!(metadata.find_ai_model_by_id(model_id).await) else {
                return ErrorResponse::ModelNotFound.into();
            };
            models.push(model);
        }
        models
    };
    let product = GeneratedProduct {
        id: GeneratedProductId::new(),
        name: request.name.into_inner(),
        models,
        mime_type: request
            .file
            .content_type
            .map(|m| m.to_string())
            .unwrap_or("application/octet-stream".to_string()),
        positive_prompt: request.positive_prompt.into_null_if_empty(),
        negative_prompt: request.negative_prompt.into_null_if_empty(),
        sampler_name: request.sampler_name.into_null_if_empty(),
        scheduler_name: request.scheduler_name.into_null_if_empty(),
        step_count: request
            .step_count
            .into_null_if_empty()
            .and_then(|s| s.parse::<i32>().ok()),
        cfg_scale: request
            .cfg_scale
            .into_null_if_empty()
            .and_then(|s| s.parse::<f32>().ok()),
        seed: request
            .seed
            .into_null_if_empty()
            .and_then(|s| s.parse::<i32>().ok()),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    let metadata = try_db!(metadata.add_generated_product(product).await);

    let mut bytes = Vec::new();
    let mut file = request.file.file;
    read_file!(file, bytes);

    try_storage!(storage.save_product(&metadata, bytes.as_slice()).await);

    HttpResponse::Created().json(AddProductResponse { product: metadata })
}
