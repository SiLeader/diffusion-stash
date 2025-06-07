use crate::endpoints::ErrorResponse;
use crate::{DataStorage, try_db, try_storage};
use actix_web::HttpResponse;
use actix_web::http::header::{CONTENT_DISPOSITION, CONTENT_TYPE};
use metadata_database::MetadataDatabase;
use metadata_database::data::GeneratedProductId;

pub(crate) async fn create_product_response(
    metadata: &MetadataDatabase,
    storage: &DataStorage,
    product_id: GeneratedProductId,
) -> HttpResponse {
    let Some(metadata) = try_db!(metadata.find_generated_product_by_id(product_id).await) else {
        return ErrorResponse::ProductNotFound.into();
    };
    let data = try_storage!(storage.load_product(&metadata).await);

    HttpResponse::Ok()
        .insert_header((CONTENT_TYPE, metadata.mime_type))
        .insert_header((CONTENT_DISPOSITION, metadata.name))
        .body(data)
}
