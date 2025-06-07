use crate::endpoints::Size;
use crate::endpoints::error_response::ErrorResponse;
use actix_web::web::{Data, Query};
use actix_web::{HttpResponse, get};
use log::error;
use metadata_database::MetadataDatabase;
use metadata_database::data::{Model, ModelBase, ModelType};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Default)]
pub(super) struct QueryParams {
    #[serde(default, flatten)]
    size: Size,
    #[serde(default)]
    query: Option<String>,
    #[serde(default)]
    category: Option<ModelBase>,
    #[serde(default, rename = "type")]
    model_type: Option<ModelType>,
}

#[derive(Serialize)]
struct ModelListResponse {
    models: Vec<Model>,
    total: usize,
}

#[get("/v1/models")]
pub(super) async fn handle_list_models(
    query: Query<QueryParams>,
    metadata: Data<MetadataDatabase>,
) -> HttpResponse {
    let query = query.into_inner();

    let res = metadata
        .find_ai_model_with_filter(
            query.query,
            query.category,
            query.model_type,
            query.size.to_offset(),
            query.size.to_limit(),
        )
        .await;
    match res {
        Ok((total, models)) => HttpResponse::Ok().json(ModelListResponse { models, total }),
        Err(e) => {
            error!("Error while listing models: {:?}", e);
            ErrorResponse::Unknown.into()
        }
    }
}
