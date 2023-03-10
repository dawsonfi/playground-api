use crate::model::account::{Account, AccountStatus, AccountType};
use crate::repository::dynamo_client::DynamoDbClient;
use crate::repository::DatabaseClient;
use aws_config::SdkConfig;
use aws_sdk_dynamodb::model::AttributeValue;
use std::error::Error;

static TABLE_NAME: &str = "Account";
static ACCOUNT_TYPE_PARAMETER: &str = "account_type";
static ACCOUNT_TYPE_VALUE: &str = "type";
static ACCOUNT_STATUS_PARAMETER: &str = "account_status";
static ACCOUNT_STATUS_VALUE: &str = "status";

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
    ) -> Result<Vec<Account>, Box<dyn Error>> {
        let mut conditions = vec![];

        let query_attributes = vec![
            (
                account_type.map(|item| item.to_string()),
                ACCOUNT_TYPE_PARAMETER,
                ACCOUNT_TYPE_VALUE,
            ),
            (
                account_status.map(|item| item.to_string()),
                ACCOUNT_STATUS_PARAMETER,
                ACCOUNT_STATUS_VALUE,
            ),
        ];

        for (input, parameter, value) in query_attributes {
            if input.is_some() {
                conditions.push((
                    format!("#{parameter} = :{value}"),
                    value.to_string(),
                    AttributeValue::S(input.unwrap()),
                ))
            }
        }

        self.client.list(TABLE_NAME.to_string(), conditions).await
    }
}
