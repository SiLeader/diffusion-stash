use crate::endpoints::add_model::handle_add_model;
use crate::endpoints::get_model_content::handle_get_model_content;
use crate::endpoints::get_product_content::handle_get_product_content;
use crate::endpoints::list_model_products::handle_list_model_products;
use crate::endpoints::list_models::handle_list_models;
use actix_web::web::ServiceConfig;
use serde::Deserialize;

mod add_model;
mod error_response;
mod get_model_content;
mod get_product_content;
mod list_model_products;
mod list_models;

pub(crate) fn register_endpoints(config: &mut ServiceConfig) {
    config
        .service(handle_add_model)
        .service(handle_list_models)
        .service(handle_get_model_content)
        .service(handle_get_product_content)
        .service(handle_list_model_products);
}

#[derive(Deserialize, Debug, Clone)]
struct Size {
    offset: usize,
    limit: usize,
}

impl Default for Size {
    fn default() -> Self {
        Self {
            offset: 0,
            limit: 100,
        }
    }
}
