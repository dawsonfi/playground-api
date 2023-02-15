use crate::repository::{account_repository::AccountRepository, ConfigProvider};
use lambda_web::actix_web::{get, http::header::ContentType, HttpResponse};
use serde_json::to_string;

#[get("/accounts")]
pub async fn list_accounts() -> HttpResponse {
    let sdk_config = ConfigProvider::default().provide().await;
    let account_repository = AccountRepository::new(&sdk_config);

    let accounts = account_repository.list_accounts(None, None).await.unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(to_string(&accounts).unwrap())
}
