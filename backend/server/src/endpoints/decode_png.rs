use crate::endpoints::ErrorResponse;
use crate::try_db;
use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_web::web::Data;
use actix_web::{HttpResponse, post};
use metadata_database::data::Model;
use metadata_database::{DbErr, MetadataDatabase};
use png_workflow::{GenerationInfo, InfoExtractorError};
use serde::Serialize;

#[derive(MultipartForm)]
pub(super) struct DecodePngRequest {
    #[multipart(limit = "100MB")]
    file: TempFile,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DecodePngResponse {
    resolved_models: Vec<Model>,
    decoded: GenerationInfo,
}

#[post("/v1/png/workflow/decoder")]
pub(super) async fn handle_decode_png(
    request: MultipartForm<DecodePngRequest>,
    metadata: Data<MetadataDatabase>,
) -> HttpResponse {
    match GenerationInfo::from_path(request.file.file.path()) {
        Ok(gi) => HttpResponse::Ok().json(DecodePngResponse {
            resolved_models: try_db!(
                resolve_models_by_name(&metadata, gi.model_names.as_slice()).await
            ),
            decoded: gi,
        }),
        Err(e) => match e {
            InfoExtractorError::Png(_)
            | InfoExtractorError::Json(_)
            | InfoExtractorError::WorkflowNotFound
            | InfoExtractorError::ParametersNotFound => ErrorResponse::InvalidPngFile.into(),
            InfoExtractorError::Io(_) => ErrorResponse::Io.into(),
        },
    }
}

async fn resolve_models_by_name(
    metadata: &MetadataDatabase,
    names: &[String],
) -> Result<Vec<Model>, DbErr> {
    let mut models = Vec::new();
    for name in names {
        let model = metadata.find_ai_model_by_name(name).await?;
        if let Some(model) = model {
            models.push(model);
        }
    }
    Ok(models)
}
