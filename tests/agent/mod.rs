mod playground_api_lambda_agent;

use playground_api_lambda_agent::PlaygroundApiLamdaAgent;
use std::error::Error;
use async_trait::async_trait;
use std::env;

#[derive(Default)]
pub struct PlaygroundApiRequest {
    pub uri: String,
    pub http_method: String,
    pub payload: Option<String>,
    pub path_parameters: Option<Vec<String>>
}

pub struct PlaygroundApiResponse {
    pub status: i32,
    pub payload: String
}

#[async_trait]
pub trait PlaygroundApiAgent {
    async fn call(&self, request: PlaygroundApiRequest) -> Result<PlaygroundApiResponse, Box<dyn Error>>;
}

pub async fn build_playground_api_agent() -> Result<Box<dyn PlaygroundApiAgent>, Box<dyn Error>> {
    Ok(match env::var("RUNNING_ENV") {
        Ok(running_env) => Box::new(PlaygroundApiLamdaAgent::new(running_env).await),
        Err(_) => Box::new(PlaygroundApiLamdaAgent::new("beta".to_string()).await) //Defaults to beta
    })
}