use crate::endpoints::error_response::ErrorResponse;
use crate::storage::DataStorage;
use crate::utils::product_response::create_product_response;
use actix_web::web::{Data, Path};
use actix_web::{HttpResponse, get};
use metadata_database::MetadataDatabase;
use metadata_database::data::GeneratedProductId;

#[get("/v1/products/{id}/content")]
pub(super) async fn handle_get_product_content(
    path: Path<String>,
    storage: Data<DataStorage>,
    metadata: Data<MetadataDatabase>,
) -> HttpResponse {
    let product_id = path.into_inner();
    let Ok(product_id) = GeneratedProductId::try_from(product_id) else {
        return ErrorResponse::InvalidProductIdFormat.into();
    };

    create_product_response(&metadata, &storage, product_id).await
}
