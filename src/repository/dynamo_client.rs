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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_extract_string_when_available() {}
}
