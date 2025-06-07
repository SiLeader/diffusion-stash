use crate::endpoints::ErrorResponse;
use crate::utils::product_response::create_product_response;
use crate::{DataStorage, try_db};
use actix_web::web::{Data, Path};
use actix_web::{HttpResponse, get};
use metadata_database::MetadataDatabase;
use metadata_database::data::ModelId;

#[get("/v1/models/{model_id}/thumbnail")]
pub(super) async fn handle_get_model_thumbnail_content(
    path: Path<String>,
    storage: Data<DataStorage>,
    metadata: Data<MetadataDatabase>,
) -> HttpResponse {
    let model_id = path.into_inner();
    let Ok(model_id) = ModelId::try_from(model_id) else {
        return ErrorResponse::InvalidModelIdFormat.into();
    };

    let (_, products) = try_db!(
        metadata
            .find_generated_product_by_model(model_id, 0, 1)
            .await
    );
    if let Some(product) = products.first() {
        create_product_response(&metadata, &storage, product.id.clone()).await
    } else {
        ErrorResponse::ThumbnailNotFound.into()
    }
}
