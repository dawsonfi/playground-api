use crate::controller::account_controller as accounts;
use crate::model::account::{Account, AccountBalance, AccountStatus, AccountType};
use crate::model::{Currency, CurrencyCode};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        accounts::list_accounts
    ),
    components(
        schemas(Currency, CurrencyCode, Account, AccountBalance, AccountStatus, AccountType)
    ),
    tags(
        (name = "accounts", description = "Account management endpoints.")
    )
)]
pub struct ApiDoc;
