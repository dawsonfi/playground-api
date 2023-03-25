use crate::error::PermanentError;
use crate::model::account::{AccountStatus, AccountType};
use crate::service::account_service::AccountService;
use actix_web::http::header::ContentType;
use actix_web::web::{Data, Query};
use lambda_web::actix_web::{get, HttpResponse};
use serde::Deserialize;
use serde_json::to_string;
use tracing::info;

#[derive(Deserialize, Debug)]
pub struct Params {
    pub account_type: Option<AccountType>,
    pub account_status: Option<AccountStatus>,
}

#[tracing::instrument(skip(account_service))]
#[get("/accounts")]
pub async fn list_accounts(
    account_service: Data<AccountService>,
    params: Query<Params>,
) -> Result<HttpResponse, PermanentError> {
    info!("Listing all accounts");

    let accounts = account_service
        .list_accounts(params.account_type, params.account_status)
        .await?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(to_string(&accounts).unwrap()))
}
