use crate::endpoints::Size;
use crate::try_db;
use actix_web::web::{Data, Query};
use actix_web::{HttpResponse, get};
use metadata_database::MetadataDatabase;
use metadata_database::data::GeneratedProduct;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Default)]
pub(super) struct QueryParams {
    #[serde(default, flatten)]
    size: Size,
}

#[derive(Serialize)]
struct ProductsResponse {
    products: Vec<GeneratedProduct>,
    total: usize,
}

#[get("/v1/products")]
pub(super) async fn handle_list_products(
    query: Query<QueryParams>,
    metadata: Data<MetadataDatabase>,
) -> HttpResponse {
    let (total, products) = try_db!(
        metadata
            .find_generated_products(query.size.to_offset(), query.size.to_limit())
            .await
    );
    HttpResponse::Ok().json(ProductsResponse { total, products })
}
