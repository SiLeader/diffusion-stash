use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "code")]
pub(super) enum ErrorResponse {
    Unknown,
}

impl From<ErrorResponse> for HttpResponse {
    fn from(value: ErrorResponse) -> Self {
        match value {
            ErrorResponse::Unknown => HttpResponse::InternalServerError().json(value),
        }
    }
}
