use crate::endpoints::Size;
use crate::endpoints::error_response::ErrorResponse;
use crate::try_db;
use actix_web::web::{Data, Path, Query};
use actix_web::{HttpResponse, get};
use metadata_database::MetadataDatabase;
use metadata_database::data::{GeneratedProduct, ModelId};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

#[get("/v1/models/{model_id}/products")]
pub(super) async fn handle_list_model_products(
    query: Query<QueryParams>,
    metadata: Data<MetadataDatabase>,
    path: Path<String>,
) -> HttpResponse {
    let model_id = path.into_inner();
    let Ok(model_id) = Uuid::parse_str(&model_id) else {
        return ErrorResponse::InvalidModelIdFormat.into();
    };
    let model_id = ModelId::from(model_id);
    let (total, products) = try_db!(
        metadata
            .find_generated_product_by_model(
                model_id,
                query.size.to_offset(),
                query.size.to_limit()
            )
            .await
    );
    HttpResponse::Ok().json(ProductsResponse { total, products })
}
