use async_trait::async_trait;
use aws_config::SdkConfig;
use aws_sdk_dynamodb::{model::AttributeValue, Client};
use std::collections::HashMap;
use std::error::Error;

#[async_trait]
pub trait DatabaseClient {
    async fn query(
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
            client: Client::new(&config),
        }
    }
}

#[async_trait]
impl DatabaseClient for DynamoDbClient {
    async fn query(
        &self,
        table_name: String,
        conditions: Vec<(String, String, AttributeValue)>,
    ) -> Result<Vec<HashMap<String, AttributeValue>>, Box<dyn Error>> {
        let mut request = self.client.query().table_name(table_name);

        for (expression, attribute_name, attribute_value) in conditions {
            let expression_parameters = expression
                .split("=")
                .map(|parameter| parameter.trim())
                .collect::<Vec<&str>>();

            request = request
                .key_condition_expression(expression.clone())
                .expression_attribute_names(expression_parameters[0], attribute_name)
                .expression_attribute_values(expression_parameters[1], attribute_value);
        }

        let result = request.send().await?.items.unwrap();

        Ok(result)
    }
}
