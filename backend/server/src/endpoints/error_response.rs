use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "code")]
pub(crate) enum ErrorResponse {
    Unknown,
    CannotReadUploadedFile,
    InvalidModelIdFormat,
    InvalidProductIdFormat,
    ProductNotFound,
    ModelNotFound,
    ThumbnailNotFound,
    InvalidPngFile,
    Io,
}

impl From<ErrorResponse> for HttpResponse {
    fn from(value: ErrorResponse) -> Self {
        match value {
            ErrorResponse::Unknown => HttpResponse::InternalServerError().json(value),
            ErrorResponse::CannotReadUploadedFile => {
                HttpResponse::InternalServerError().json(value)
            }
            ErrorResponse::InvalidModelIdFormat => HttpResponse::BadRequest().json(value),
            ErrorResponse::InvalidProductIdFormat => HttpResponse::BadRequest().json(value),
            ErrorResponse::ProductNotFound => HttpResponse::NotFound().json(value),
            ErrorResponse::ModelNotFound => HttpResponse::NotFound().json(value),
            ErrorResponse::ThumbnailNotFound => HttpResponse::NotFound().json(value),
            ErrorResponse::InvalidPngFile => HttpResponse::BadRequest().json(value),
            ErrorResponse::Io => HttpResponse::InternalServerError().json(value),
        }
    }
}

#[macro_export]
macro_rules! try_db {
    ($access:expr) => {
        match $access {
            Ok(value) => value,
            Err(e) => {
                log::error!("Database error: {e:?}");
                return $crate::endpoints::ErrorResponse::Unknown.into();
            }
        }
    };
}

#[macro_export]
macro_rules! try_storage {
    ($access:expr) => {
        match $access {
            Ok(value) => value,
            Err(e) => {
                log::error!("Storage error: {e:?}");
                return $crate::endpoints::ErrorResponse::Unknown.into();
            }
        }
    };
}
