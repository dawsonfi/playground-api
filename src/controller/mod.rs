use crate::error::PermanentError;
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, ResponseError};
use serde_json::Value;

pub mod account_controller;

impl ResponseError for PermanentError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let message = format!(r#"{{"cause": "{}"}}"#, self);

        HttpResponse::Ok().content_type(ContentType::json()).body(
            serde_json::from_str::<Value>(message.as_str())
                .unwrap()
                .to_string(),
        )
    }
}
