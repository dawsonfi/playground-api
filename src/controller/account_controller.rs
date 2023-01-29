use lambda_web::actix_web::{get, http::header::ContentType, HttpResponse};

#[get("/accounts")]
pub async fn list_accounts() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body("{}")
}
