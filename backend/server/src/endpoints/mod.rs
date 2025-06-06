use crate::endpoints::add_model::handle_add_model;
use crate::endpoints::add_product::handle_add_product;
use crate::endpoints::get_model_content::handle_get_model_content;
use crate::endpoints::get_model_metadata::{handle_get_model, handle_get_model_metadata};
use crate::endpoints::get_product_content::handle_get_product_content;
use crate::endpoints::get_product_metadata::{handle_get_product, handle_get_product_metadata};
use crate::endpoints::list_model_products::handle_list_model_products;
use crate::endpoints::list_models::handle_list_models;
use actix_multipart::form::text::Text;
use actix_web::web::ServiceConfig;
use serde::Deserialize;

mod add_model;
mod add_product;
mod error_response;
mod get_model_content;
mod get_model_metadata;
mod get_product_content;
mod get_product_metadata;
mod list_model_products;
mod list_models;

pub(crate) fn register_endpoints(config: &mut ServiceConfig) {
    config
        // add
        .service(handle_add_model)
        .service(handle_add_product)
        // get metadata
        .service(handle_get_model)
        .service(handle_get_model_metadata)
        .service(handle_get_product)
        .service(handle_get_product_metadata)
        // get content
        .service(handle_get_model_content)
        .service(handle_get_product_content)
        // list
        .service(handle_list_models)
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

#[macro_export]
macro_rules! read_file {
    ($file:expr, $bytes:ident) => {
        if let Err(e) = $file.read_to_end(&mut $bytes) {
            error!("Cannot read uploaded file: {e:?}");
            return ErrorResponse::CannotReadUploadedFile.into();
        }
    };
}

trait IntoNullIfEmpty {
    fn into_null_if_empty(self) -> Option<String>;
}

impl IntoNullIfEmpty for String {
    fn into_null_if_empty(self) -> Option<String> {
        if self.is_empty() { None } else { Some(self) }
    }
}

impl IntoNullIfEmpty for Text<String> {
    fn into_null_if_empty(self) -> Option<String> {
        self.into_inner().into_null_if_empty()
    }
}

impl IntoNullIfEmpty for Option<Text<String>> {
    fn into_null_if_empty(self) -> Option<String> {
        self.and_then(|t| t.into_inner().into_null_if_empty())
    }
}
