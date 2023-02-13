use crate::agent::{PlaygroundApiAgent, PlaygroundApiRequest, PlaygroundApiResponse};
use async_trait::async_trait;
use aws_sdk_lambda::{model::InvocationType, Client};
use aws_smithy_types::Blob;
use base64::{engine::general_purpose, Engine as _};
use serde_json::{from_str, to_string, Map, Value};
use std::error::Error;

pub struct PlaygroundApiLamdaAgent {
    client: Client,
    function_name: String,
}

impl PlaygroundApiLamdaAgent {
    #[cfg(feature = "integration")]
    pub async fn new(running_env: String) -> Self {
        let config = aws_config::load_from_env().await;
        let client = Client::new(&config);

        PlaygroundApiLamdaAgent {
            client: client,
            function_name: format!("{}{}", running_env, "-playground-lambda-api"),
        }
    }

    fn build_request_payload(request: PlaygroundApiRequest) -> Blob {
        let mut payload = Map::new();

        payload.insert("path".to_string(), Value::String(request.uri));
        payload.insert("httpMethod".to_string(), Value::String(request.http_method));
        payload.insert("multiValueHeaders".to_string(), Value::Object(Map::new()));
        payload.insert("requestContext".to_string(), Value::Object(Map::new()));

        Blob::new(to_string(&Value::Object(payload)).unwrap())
    }

    fn extract_response_payload(payload: &Blob) -> String {
        let body = String::from_utf8(payload.as_ref().to_vec()).unwrap();
        let body_json: Value = from_str(body.as_str()).unwrap();
        let body_decoded_bytes = general_purpose::STANDARD
            .decode(body_json["body"].as_str().unwrap())
            .unwrap();
        String::from_utf8(body_decoded_bytes).unwrap()
    }
}

#[async_trait]
impl PlaygroundApiAgent for PlaygroundApiLamdaAgent {
    async fn call(
        &self,
        request: PlaygroundApiRequest,
    ) -> Result<PlaygroundApiResponse, Box<dyn Error>> {
        let resp = self
            .client
            .invoke()
            .function_name(self.function_name.clone())
            .invocation_type(InvocationType::RequestResponse)
            .payload(PlaygroundApiLamdaAgent::build_request_payload(request))
            .send()
            .await
            .unwrap();

        let body_decoded =
            PlaygroundApiLamdaAgent::extract_response_payload(resp.payload().unwrap());
        Ok(PlaygroundApiResponse {
            status: resp.status_code(),
            payload: body_decoded,
        })
    }
}
