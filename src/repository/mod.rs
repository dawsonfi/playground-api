mod account_repository;
mod dynamo_client;

use crate::error::PermanentError;
use crate::model::account::Account;
pub use account_repository::AccountRepository;
use async_trait::async_trait;
use aws_config::{from_env, SdkConfig};
use aws_credential_types::cache::CredentialsCache;
#[cfg(test)]
use mockall::automock;

#[derive(Default)]
pub struct ConfigProvider {}

impl ConfigProvider {
    pub async fn provide(&self) -> SdkConfig {
        from_env()
            .credentials_cache(CredentialsCache::lazy())
            .load()
            .await
    }
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait DatabaseClient {
    async fn list(
        &self,
        table_name: String,
        conditions: Vec<(String, String)>,
    ) -> Result<Vec<Account>, PermanentError>;
}
