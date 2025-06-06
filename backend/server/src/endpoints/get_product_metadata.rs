use crate::endpoints::error_response::ErrorResponse;
use crate::try_db;
use actix_web::web::{Data, Path};
use actix_web::{HttpResponse, get};
use metadata_database::MetadataDatabase;
use metadata_database::data::{GeneratedProduct, GeneratedProductId};
use serde::Serialize;

#[get("/v1/products/{product_id}")]
pub(super) async fn handle_get_product(
    path: Path<String>,
    metadata: Data<MetadataDatabase>,
) -> HttpResponse {
    handle_get_product_metadata_impl(path, metadata).await
}

#[get("/v1/products/{product_id}/metadata")]
pub(super) async fn handle_get_product_metadata(
    path: Path<String>,
    metadata: Data<MetadataDatabase>,
) -> HttpResponse {
    handle_get_product_metadata_impl(path, metadata).await
}

#[derive(Serialize)]
struct ProductMetadataResponse {
    product: GeneratedProduct,
}

async fn handle_get_product_metadata_impl(
    path: Path<String>,
    metadata: Data<MetadataDatabase>,
) -> HttpResponse {
    let product_id = path.into_inner();
    let Ok(product_id) = GeneratedProductId::try_from(product_id) else {
        return ErrorResponse::InvalidProductIdFormat.into();
    };

    let Some(metadata) = try_db!(
        metadata
            .find_generated_product_by_id(product_id.clone())
            .await
    ) else {
        return ErrorResponse::ProductNotFound.into();
    };

    HttpResponse::Ok().json(ProductMetadataResponse { product: metadata })
}
