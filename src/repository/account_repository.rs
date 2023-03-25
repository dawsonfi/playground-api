use crate::error::PermanentError;
use crate::model::account::{Account, AccountStatus, AccountType};
use crate::repository::dynamo_client::DynamoDbClient;
use crate::repository::DatabaseClient;
use aws_config::SdkConfig;

static TABLE_NAME: &str = "Account";
static ACCOUNT_TYPE_PARAMETER: &str = "type";
static ACCOUNT_STATUS_PARAMETER: &str = "status";

pub struct AccountRepository {
    client: Box<dyn DatabaseClient>,
}

impl AccountRepository {
    pub fn new(config: &SdkConfig) -> Self {
        AccountRepository {
            client: Box::new(DynamoDbClient::new(config)),
        }
    }

    pub async fn list_accounts(
        &self,
        account_type: Option<AccountType>,
        account_status: Option<AccountStatus>,
    ) -> Result<Vec<Account>, PermanentError> {
        let query_attributes = vec![
            (
                ACCOUNT_TYPE_PARAMETER.to_string(),
                account_type.map(|item| item.to_string()),
            ),
            (
                ACCOUNT_STATUS_PARAMETER.to_string(),
                account_status.map(|item| item.to_string()),
            ),
        ]
        .into_iter()
        .filter(|(_, value)| value.is_some())
        .map(|(parameter, value)| (parameter, value.unwrap()))
        .collect();

        self.client
            .list(TABLE_NAME.to_string(), query_attributes)
            .await
    }
}
