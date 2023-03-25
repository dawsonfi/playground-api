use crate::error::PermanentError;
use crate::model::account::{Account, AccountBalance, AccountStatus, AccountType};
use crate::model::{Currency, CurrencyCode};
use crate::repository::DatabaseClient;
use async_trait::async_trait;
use aws_config::SdkConfig;
use aws_sdk_dynamodb::error::ScanError;
use aws_sdk_dynamodb::types::SdkError;
use aws_sdk_dynamodb::{model::AttributeValue, Client};
use chrono::NaiveDateTime;
use std::collections::HashMap;
use std::str::FromStr;

pub struct DynamoDbClient {
    client: Client,
}

impl DynamoDbClient {
    pub fn new(config: &SdkConfig) -> Self {
        DynamoDbClient {
            client: Client::new(config),
        }
    }
}

#[async_trait]
impl DatabaseClient for DynamoDbClient {
    #[tracing::instrument(skip(self))]
    async fn list(
        &self,
        table_name: String,
        conditions: Vec<(String, String)>,
    ) -> Result<Vec<Account>, PermanentError> {
        let mut request = self.client.scan().table_name(table_name);

        let mut filter_expression: Option<String> = None;
        for (attribute_name, attribute_value) in conditions {
            let name_parameter = format!("#{attribute_name}");
            let value_parameter = format!(":{attribute_name}");

            filter_expression = Some(match filter_expression {
                Some(expression) => {
                    format!("{expression} AND {name_parameter} = {value_parameter}")
                }
                None => format!("{name_parameter} = {value_parameter}"),
            });

            request = request
                .filter_expression(filter_expression.as_ref().unwrap())
                .expression_attribute_names(name_parameter, attribute_name)
                .expression_attribute_values(value_parameter, AttributeValue::S(attribute_value));
        }

        let result = request
            .send()
            .await
            .map_err(PermanentError::from)?
            .items
            .unwrap();

        Ok(result
            .into_iter()
            .map(Account::from)
            .collect::<Vec<Account>>())
    }
}

impl DynamoDbClient {
    pub fn extract_string(key: &str, values: &HashMap<String, AttributeValue>) -> Option<String> {
        Some(values.get(key)?.as_s().unwrap().clone())
    }

    pub fn extract_number(key: &str, values: &HashMap<String, AttributeValue>) -> Option<f64> {
        Some(values.get(key)?.as_n().unwrap().parse::<f64>().unwrap())
    }

    pub fn extract_list(
        key: &str,
        values: &HashMap<String, AttributeValue>,
    ) -> Option<Vec<AttributeValue>> {
        Some(values.get(key)?.as_l().unwrap().clone())
    }

    pub fn extract_map(
        key: &str,
        values: &HashMap<String, AttributeValue>,
    ) -> Option<HashMap<String, AttributeValue>> {
        Some(values.get(key)?.as_m().unwrap().clone())
    }

    pub fn extract_bool(key: &str, values: &HashMap<String, AttributeValue>) -> Option<bool> {
        Some(*values.get(key)?.as_bool().unwrap())
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

impl From<SdkError<ScanError>> for PermanentError {
    fn from(value: SdkError<ScanError>) -> Self {
        let service_error = value.into_service_error();
        Self {
            message: service_error.message().map(String::from),
            source: Box::new(service_error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_extract_string_when_available() {
        let key = "batata";
        let mut values = HashMap::new();
        values.insert(key.to_string(), AttributeValue::S("frita".to_string()));

        let result = DynamoDbClient::extract_string(key, &values);

        assert!(result.is_some());
        assert_eq!("frita".to_string(), result.unwrap());
    }

    #[test]
    fn should_return_none_when_string_is_not_available() {
        let key = "batata";
        let values = HashMap::new();

        let result = DynamoDbClient::extract_string(key, &values);

        assert!(result.is_none());
    }

    #[test]
    #[should_panic]
    fn should_fail_when_attribute_is_not_string() {
        let key = "batata";
        let mut values = HashMap::new();
        values.insert(key.to_string(), AttributeValue::N("1".to_string()));

        DynamoDbClient::extract_string(key, &values);
    }

    #[test]
    fn should_extract_number_when_available() {
        let key = "batata";
        let mut values = HashMap::new();
        values.insert(key.to_string(), AttributeValue::N("1.1".to_string()));

        let result = DynamoDbClient::extract_number(key, &values);

        assert!(result.is_some());
        assert_eq!(1.1, result.unwrap());
    }

    #[test]
    fn should_return_none_when_number_is_not_available() {
        let key = "batata";
        let values = HashMap::new();

        let result = DynamoDbClient::extract_number(key, &values);

        assert!(result.is_none());
    }

    #[test]
    #[should_panic]
    fn should_fail_when_attribute_is_not_number() {
        let key = "batata";
        let mut values = HashMap::new();
        values.insert(key.to_string(), AttributeValue::S("batata".to_string()));

        DynamoDbClient::extract_number(key, &values);
    }

    #[test]
    #[should_panic]
    fn should_fail_when_attribute_is_not_parseable_number() {
        let key = "batata";
        let mut values = HashMap::new();
        values.insert(key.to_string(), AttributeValue::N("batata".to_string()));

        DynamoDbClient::extract_number(key, &values);
    }

    #[test]
    fn should_extract_bool_when_available() {
        let key = "batata";
        let mut values = HashMap::new();
        values.insert(key.to_string(), AttributeValue::Bool(true));

        let result = DynamoDbClient::extract_bool(key, &values);

        assert!(result.is_some());
        assert!(result.unwrap());
    }

    #[test]
    fn should_return_none_when_bool_is_not_available() {
        let key = "batata";
        let values = HashMap::new();

        let result = DynamoDbClient::extract_bool(key, &values);

        assert!(result.is_none());
    }

    #[test]
    #[should_panic]
    fn should_fail_when_attribute_is_not_bool() {
        let key = "batata";
        let mut values = HashMap::new();
        values.insert(key.to_string(), AttributeValue::N("1".to_string()));

        DynamoDbClient::extract_bool(key, &values);
    }

    #[test]
    fn should_extract_list_when_available() {
        let key = "batata";
        let mut values = HashMap::new();
        values.insert(
            key.to_string(),
            AttributeValue::L(vec![AttributeValue::S("test".to_string())]),
        );

        let result = DynamoDbClient::extract_list(key, &values);

        assert!(result.is_some());
        assert_eq!(vec![AttributeValue::S("test".to_string())], result.unwrap());
    }

    #[test]
    fn should_return_none_when_list_is_not_available() {
        let key = "batata";
        let values = HashMap::new();

        let result = DynamoDbClient::extract_list(key, &values);

        assert!(result.is_none());
    }

    #[test]
    #[should_panic]
    fn should_fail_when_attribute_is_not_list() {
        let key = "batata";
        let mut values = HashMap::new();
        values.insert(key.to_string(), AttributeValue::N("1".to_string()));

        DynamoDbClient::extract_list(key, &values);
    }

    #[test]
    fn should_extract_map_when_available() {
        let key = "batata";

        let mut map = HashMap::new();
        map.insert("batata".to_string(), AttributeValue::Bool(true));

        let mut values = HashMap::new();
        values.insert(key.to_string(), AttributeValue::M(map.clone()));

        let result = DynamoDbClient::extract_map(key, &values);

        assert!(result.is_some());
        assert_eq!(map.clone(), result.unwrap());
    }

    #[test]
    fn should_return_none_when_map_is_not_available() {
        let key = "batata";
        let values = HashMap::new();

        let result = DynamoDbClient::extract_map(key, &values);

        assert!(result.is_none());
    }

    #[test]
    #[should_panic]
    fn should_fail_when_attribute_is_not_map() {
        let key = "batata";
        let mut values = HashMap::new();
        values.insert(key.to_string(), AttributeValue::N("1".to_string()));

        DynamoDbClient::extract_map(key, &values);
    }

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
