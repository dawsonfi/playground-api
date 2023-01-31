#[cfg(feature = "integration")]
#[cfg(test)]
mod tests {
    use actix_web::{http::header::ContentType, test, App};
    use playground_api::controller::account_controller::list_accounts;
    use serde_json::{Value};

    #[actix_web::test]
    async fn test_index_get() {
        let app = test::init_service(App::new().service(list_accounts)).await;
        let req = test::TestRequest::default()
            .uri("/accounts")
            .insert_header(ContentType::json())
            .to_request();
        let resp: Value = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp.to_string(), "{}");
    }
}
