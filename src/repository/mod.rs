pub mod account_repository;
pub mod dynamo_client;

use aws_config::{load_from_env, SdkConfig};

#[derive(Default)]
pub struct ConfigProvider {}

impl ConfigProvider {
    pub async fn provide(&self) -> SdkConfig {
        load_from_env().await
    }
}
