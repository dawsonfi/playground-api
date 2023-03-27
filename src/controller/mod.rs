use crate::error::PermanentError;
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, ResponseError};
use serde_json::Value;

pub mod account_controller;

impl ResponseError for PermanentError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let message = format!(r#"{{"cause": "{}"}}"#, self);

        HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body(
                serde_json::from_str::<Value>(message.as_str())
                    .unwrap()
                    .to_string(),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::body::MessageBody;
    use actix_web::http::StatusCode;
    use std::error::Error;
    use std::fmt::{Display, Formatter};

    #[test]
    fn should_return_internal_server_error() {
        let error = PermanentError {
            source: Box::new(TestError {}),
            message: Some("test".to_string()),
        };

        let error_response = error.error_response();

        assert_eq!(error_response.status(), StatusCode::INTERNAL_SERVER_ERROR);

        assert_eq!(
            error_response
                .headers()
                .get("Content-Type")
                .unwrap()
                .to_str()
                .unwrap(),
            "application/json"
        );

        assert_eq!(
            format!("{:?}", error_response.into_body().try_into_bytes().unwrap()),
            "b\"{\\\"cause\\\":\\\"error: test\\\"}\""
        );
    }

    #[derive(Debug)]
    struct TestError {}

    impl Display for TestError {
        fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
            write!(formatter, "error")
        }
    }

    impl Error for TestError {}
}
