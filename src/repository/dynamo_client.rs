use async_trait::async_trait;
use aws_config::SdkConfig;
use aws_sdk_dynamodb::model::ConditionalOperator;
use aws_sdk_dynamodb::{model::AttributeValue, Client};
#[cfg(test)]
use mockall::automock;
use std::collections::HashMap;
use std::error::Error;

#[async_trait]
#[cfg_attr(test, automock)]
pub trait DatabaseClient {
    async fn scan(
        &self,
        table_name: String,
        conditions: Vec<(String, String, AttributeValue)>,
    ) -> Result<Vec<HashMap<String, AttributeValue>>, Box<dyn Error>>;
}

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
    async fn scan(
        &self,
        table_name: String,
        conditions: Vec<(String, String, AttributeValue)>,
    ) -> Result<Vec<HashMap<String, AttributeValue>>, Box<dyn Error>> {
        let mut request = self.client.scan().table_name(table_name);

        for (expression, attribute_name, attribute_value) in conditions {
            let expression_parameters = expression
                .split('=')
                .map(|parameter| parameter.trim())
                .collect::<Vec<&str>>();

            request = request
                .filter_expression(expression.clone())
                .conditional_operator(ConditionalOperator::And)
                .expression_attribute_names(expression_parameters[0], attribute_name)
                .expression_attribute_values(expression_parameters[1], attribute_value);
        }

        let result = request.send().await?.items.unwrap();

        Ok(result)
    }
}

impl DynamoDbClient {
    pub fn extract_string(key: &str, values: &HashMap<String, AttributeValue>) -> Option<String> {
        Some(values.get(key)?.as_s().unwrap().clone())
    }

    pub fn extract_number(key: &str, values: &HashMap<String, AttributeValue>) -> Option<f32> {
        Some(values.get(key)?.as_n().unwrap().parse::<f32>().unwrap())
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
}
