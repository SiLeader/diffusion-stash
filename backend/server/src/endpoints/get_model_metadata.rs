use crate::endpoints::error_response::ErrorResponse;
use crate::try_db;
use actix_web::web::{Data, Path};
use actix_web::{HttpResponse, get};
use metadata_database::MetadataDatabase;
use metadata_database::data::{Model, ModelId};
use serde::Serialize;

#[get("/v1/models/{model_id}")]
pub(super) async fn handle_get_model(
    path: Path<String>,
    metadata: Data<MetadataDatabase>,
) -> HttpResponse {
    handle_get_model_metadata_impl(path, metadata).await
}

#[get("/v1/models/{model_id}/metadata")]
pub(super) async fn handle_get_model_metadata(
    path: Path<String>,
    metadata: Data<MetadataDatabase>,
) -> HttpResponse {
    handle_get_model_metadata_impl(path, metadata).await
}

#[derive(Serialize)]
struct ModelMetadataResponse {
    model: Model,
}

async fn handle_get_model_metadata_impl(
    path: Path<String>,
    metadata: Data<MetadataDatabase>,
) -> HttpResponse {
    let model_id = path.into_inner();
    let Ok(model_id) = ModelId::try_from(model_id) else {
        return ErrorResponse::InvalidModelIdFormat.into();
    };

    let Some(metadata) = try_db!(metadata.find_ai_model_by_id(model_id.clone()).await) else {
        return ErrorResponse::ModelNotFound.into();
    };

    HttpResponse::Ok().json(ModelMetadataResponse { model: metadata })
}
