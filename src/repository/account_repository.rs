use crate::model::account::{Account, AccountBalance, AccountStatus, AccountType};
use crate::model::{Currency, CurrencyCode};
use crate::repository::dynamo_client::{DatabaseClient, DynamoDbClient};
use aws_config::SdkConfig;
use aws_sdk_dynamodb::model::AttributeValue;
use chrono::NaiveDateTime;
use std::collections::HashMap;
use std::convert::From;
use std::error::Error;
use std::str::FromStr;

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

        let result = self
            .client
            .scan(TABLE_NAME.to_string(), conditions)
            .await?
            .into_iter()
            .map(Account::from)
            .collect::<Vec<Account>>();

        Ok(result)
    }
}

impl From<HashMap<String, AttributeValue>> for Account {
    fn from(values: HashMap<String, AttributeValue>) -> Self {
        Account {
            id: DynamoDbClient::extract_string("id", &values).unwrap(),
            name: DynamoDbClient::extract_string("name", &values).unwrap(),
            bank_name: DynamoDbClient::extract_string("bank_name", &values).unwrap(),
            open_date: DynamoDbClient::extract_string("open_date", &values)
                .map(convert_date)
                .unwrap(),
            close_date: DynamoDbClient::extract_string("close_date", &values).map(convert_date),
            account_type: AccountType::from_str(
                DynamoDbClient::extract_string("type", &values)
                    .unwrap()
                    .as_str(),
            )
            .unwrap(),
            balances: DynamoDbClient::extract_list("balances", &values)
                .unwrap()
                .into_iter()
                .map(|attribute| attribute.as_m().unwrap().clone())
                .map(AccountBalance::from)
                .collect(),
            status: AccountStatus::from_str(
                DynamoDbClient::extract_string("status", &values)
                    .unwrap()
                    .as_str(),
            )
            .unwrap(),
        }
    }
}

impl From<HashMap<String, AttributeValue>> for AccountBalance {
    fn from(values: HashMap<String, AttributeValue>) -> Self {
        AccountBalance {
            date: DynamoDbClient::extract_string("date", &values)
                .map(convert_date)
                .unwrap(),
            balance: Currency::from(DynamoDbClient::extract_map("balance", &values).unwrap()),
        }
    }
}

impl From<HashMap<String, AttributeValue>> for Currency {
    fn from(values: HashMap<String, AttributeValue>) -> Self {
        Currency {
            code: CurrencyCode::from_str(
                DynamoDbClient::extract_string("currency", &values)
                    .unwrap()
                    .as_str(),
            )
            .unwrap(),
            value: DynamoDbClient::extract_number("value", &values).unwrap(),
        }
    }
}

fn convert_date(value: String) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(value.as_str(), "%d/%m/%Y %H:%M:%S%z").unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_valid_date() {
        let expected_date =
            NaiveDateTime::parse_from_str("15/02/2023 13:51:12+03:00", "%d/%m/%Y %H:%M:%S%z")
                .unwrap();

        assert_eq!(
            expected_date,
            convert_date("15/02/2023 13:51:12+03:00".to_string())
        )
    }

    #[test]
    #[should_panic]
    fn should_panic_when_converting_invalid_date() {
        convert_date("15/02/2023 13:51".to_string());
    }
}
