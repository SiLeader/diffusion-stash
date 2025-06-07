use crate::endpoints::IntoNullIfEmpty;
use crate::endpoints::error_response::ErrorResponse;
use crate::storage::DataStorage;
use crate::{read_file, try_db, try_storage};
use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_web::web::Data;
use actix_web::{HttpResponse, post};
use chrono::Utc;
use log::error;
use metadata_database::MetadataDatabase;
use metadata_database::data::{Model, ModelBase, ModelId, ModelType};
use serde::Serialize;
use std::io::Read;

#[derive(MultipartForm)]
pub(super) struct AddModelRequest {
    #[multipart(limit = "20GB")]
    file: TempFile,
    name: Text<String>,
    description: Option<Text<String>>,
    base_model: Option<Text<ModelBase>>,
    model_type: Option<Text<ModelType>>,
}

#[derive(Serialize)]
struct AddModelResponse {
    model: Model,
}

#[post("/v1/models")]
pub(super) async fn handle_add_model(
    request: MultipartForm<AddModelRequest>,
    metadata: Data<MetadataDatabase>,
    storage: Data<DataStorage>,
) -> HttpResponse {
    let request = request.into_inner();
    if request.model_type.is_none() {
        return HttpResponse::NotImplemented().finish();
    }
    let metadata = try_db!(
        metadata
            .add_ai_model(Model {
                id: ModelId::new(),
                description: request.description.into_null_if_empty().unwrap_or_default(),
                file_name: request.file.file_name.unwrap_or(request.name.clone()),
                name: request.name.into_inner(),
                base_model: request.base_model.map(|c| c.into_inner()),
                model_type: request.model_type.map(|t| t.into_inner()),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
            .await
    );

    let mut bytes = Vec::new();
    let mut file = request.file.file;
    read_file!(file, bytes);
    try_storage!(storage.save_model(&metadata, bytes.as_slice()).await);

    HttpResponse::Created().json(AddModelResponse { model: metadata })
}
