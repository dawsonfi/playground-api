mod playground_api_lambda_agent;

use async_trait::async_trait;
#[cfg(feature = "integration")]
use playground_api_lambda_agent::PlaygroundApiLambdaAgent;
#[cfg(feature = "integration")]
use std::env;
use std::error::Error;

#[derive(Default)]
pub struct PlaygroundApiRequest {
    pub uri: String,
    pub http_method: String,
    pub payload: Option<String>,
    pub path_parameters: Option<Vec<String>>,
}

pub struct PlaygroundApiResponse {
    pub status: i32,
    pub payload: String,
}

#[async_trait]
pub trait PlaygroundApiAgent {
    async fn call(
        &self,
        request: PlaygroundApiRequest,
    ) -> Result<PlaygroundApiResponse, Box<dyn Error>>;
}

#[cfg(feature = "integration")]
pub async fn build_playground_api_agent() -> Result<Box<dyn PlaygroundApiAgent>, Box<dyn Error>> {
    Ok(match env::var("RUNNING_ENV") {
        Ok(running_env) => Box::new(PlaygroundApiLambdaAgent::new(running_env).await),
        Err(_) => Box::new(PlaygroundApiLambdaAgent::new("beta".to_string()).await), //Defaults to beta
    })
}
