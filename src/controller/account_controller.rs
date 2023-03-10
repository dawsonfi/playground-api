use crate::service::account_service::AccountService;
use actix_web::web::Data;
use lambda_web::actix_web::{get, http::header::ContentType, HttpResponse};
use serde_json::to_string;

#[get("/accounts")]
pub async fn list_accounts(account_service: Data<AccountService>) -> HttpResponse {
    let accounts = account_service.list_accounts(None, None).await.unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(to_string(&accounts).unwrap())
}
