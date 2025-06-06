use crate::endpoints::error_response::ErrorResponse;
use crate::storage::DataStorage;
use crate::{try_db, try_storage};
use actix_web::http::header::{CONTENT_DISPOSITION, CONTENT_TYPE};
use actix_web::web::{Data, Path};
use actix_web::{HttpResponse, get};
use metadata_database::MetadataDatabase;
use metadata_database::data::ModelId;
use uuid::Uuid;

#[get("/v1/models/{id}/content")]
pub(super) async fn handle_get_model_content(
    path: Path<String>,
    storage: Data<DataStorage>,
    metadata: Data<MetadataDatabase>,
) -> HttpResponse {
    let model_id = path.into_inner();
    let Ok(model_id) = Uuid::parse_str(&model_id) else {
        return ErrorResponse::InvalidModelIdFormat.into();
    };
    let model_id = ModelId::from(model_id);

    let Some(metadata) = try_db!(metadata.find_ai_model_by_id(model_id.clone()).await) else {
        return ErrorResponse::ModelNotFound.into();
    };
    let data = try_storage!(storage.load_model(&metadata).await);

    HttpResponse::Ok()
        .insert_header((CONTENT_TYPE, "application/octet-stream"))
        .insert_header((CONTENT_DISPOSITION, metadata.file_name))
        .body(data)
}
