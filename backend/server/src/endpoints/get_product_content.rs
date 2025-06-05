use crate::endpoints::error_response::ErrorResponse;
use crate::storage::DataStorage;
use crate::{try_db, try_storage};
use actix_web::http::header::{CONTENT_DISPOSITION, CONTENT_TYPE};
use actix_web::web::{Data, Path};
use actix_web::{HttpResponse, get};
use metadata_database::MetadataDatabase;
use metadata_database::data::GeneratedProductId;
use uuid::Uuid;

#[get("/v1/products/{id}/content")]
pub(super) async fn handle_get_product_content(
    path: Path<String>,
    storage: Data<DataStorage>,
    metadata: Data<MetadataDatabase>,
) -> HttpResponse {
    let product_id = path.into_inner();
    let Ok(product_id) = Uuid::parse_str(&product_id) else {
        return ErrorResponse::InvalidProductIdFormat.into();
    };
    let product_id = GeneratedProductId::from(product_id);

    let Some(metadata) = try_db!(
        metadata
            .find_generated_product_by_id(product_id.clone())
            .await
    ) else {
        return ErrorResponse::ProductNotFound.into();
    };
    let data = try_storage!(storage.load_product(&product_id).await);

    HttpResponse::Ok()
        .insert_header((CONTENT_TYPE, metadata.mime_type))
        .insert_header((CONTENT_DISPOSITION, metadata.name))
        .body(data)
}
