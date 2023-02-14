use crate::repository::{account_repository::AccountRepository, ConfigProvider};
use lambda_web::actix_web::{get, http::header::ContentType, HttpResponse};

#[get("/accounts")]
pub async fn list_accounts() -> HttpResponse {
    let sdk_config = ConfigProvider::default().provide().await;
    let _account_repository = AccountRepository::new(&sdk_config);

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body("{}")
}
