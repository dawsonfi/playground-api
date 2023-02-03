// #[cfg(feature = "integration")]
#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    use playground_api::controller::account_controller::list_accounts;
    use serde_json::{Value, from_str};

    use aws_sdk_lambda::{Error, Client, model::InvocationType};
    use aws_smithy_types::Blob;
    use base64::{Engine as _, engine::general_purpose};

    #[actix_web::test]
    async fn test_index_get() {
        let app = test::init_service(App::new().service(list_accounts)).await;
        let req = test::TestRequest::default()
            .uri("/accounts")
            .to_request();
        let resp: Value = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp.to_string(), "{}");
    }

    #[actix_web::test]
    async fn test_index_get_lambda() -> Result<(), Error> {
        let config = aws_config::load_from_env().await;
        let client = Client::new(&config);

        let resp = client
            .invoke()
            .function_name("dawfre-playground-lambda-api")
            .invocation_type(InvocationType::RequestResponse)
            .payload(Blob::new("{\"path\": \"/accounts\", \"httpMethod\": \"GET\", \"multiValueHeaders\": {}, \"requestContext\": {}}"))
            .send()
            .await?;

        let payload = resp.payload().unwrap();
        let body = String::from_utf8(payload.as_ref().to_vec()).unwrap();
        let body_json: Value = from_str(body.as_str()).unwrap();
        let body_decoded_bytes = general_purpose::STANDARD.decode(body_json["body"].as_str().unwrap()).unwrap();
        let body_decoded = String::from_utf8(body_decoded_bytes).unwrap();
        assert_eq!(body_decoded, "{}");

        Ok(())
    }
}
