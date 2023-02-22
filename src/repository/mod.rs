pub mod account_repository;
pub mod dynamo_client;

use crate::model::account::Account;
use async_trait::async_trait;
use aws_config::{load_from_env, SdkConfig};
use aws_sdk_dynamodb::model::AttributeValue;
#[cfg(test)]
use mockall::automock;
use std::error::Error;

#[derive(Default)]
pub struct ConfigProvider {}

impl ConfigProvider {
    pub async fn provide(&self) -> SdkConfig {
        load_from_env().await
    }
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait DatabaseClient {
    async fn list(
        &self,
        table_name: String,
        conditions: Vec<(String, String, AttributeValue)>,
    ) -> Result<Vec<Account>, Box<dyn Error>>;
}
