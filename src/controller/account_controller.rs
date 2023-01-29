use lambda_web::actix_web::{get, Responder};

#[get("/accounts")]
pub async fn list_accounts() -> impl Responder {
    format!("TODO")
}