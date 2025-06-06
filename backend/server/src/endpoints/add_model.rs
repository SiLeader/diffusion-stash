use crate::endpoints::error_response::ErrorResponse;
use crate::storage::DataStorage;
use crate::{try_db, try_storage};
use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_web::web::Data;
use actix_web::{HttpResponse, post};
use chrono::Utc;
use log::error;
use metadata_database::MetadataDatabase;
use metadata_database::data::{Model, ModelCategory, ModelId, ModelType};
use serde::Serialize;
use std::io::Read;
use storage::Storage;

#[derive(MultipartForm)]
pub(super) struct AddModelRequest {
    file: TempFile,
    name: Text<String>,
    category: Option<Text<ModelCategory>>,
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
    let metadata = try_db!(
        metadata
            .add_ai_model(Model {
                id: ModelId::new(),
                file_name: request.file.file_name.unwrap_or(request.name.clone()),
                name: request.name.into_inner(),
                category: request.category.map(|c| c.into_inner()),
                model_type: request.model_type.map(|t| t.into_inner()),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
            .await
    );
    let bytes = {
        let mut bytes = Vec::new();
        let mut file = request.file.file;
        if let Err(e) = file.read_to_end(&mut bytes) {
            error!("Cannot read uploaded file: {e:?}");
            return ErrorResponse::CannotReadUploadedFile.into();
        }
        bytes
    };
    try_storage!(storage.save_model(&metadata, bytes.as_slice()).await);

    HttpResponse::Created().json(AddModelResponse { model: metadata })
}
