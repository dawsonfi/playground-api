mod agent;

#[cfg(feature = "integration")]
#[cfg(test)]
mod tests {
    use crate::agent::{build_playground_api_agent, PlaygroundApiRequest};
    use playground_api::model::account::Account;
    use serde_json::from_str;

    #[tokio::test]
    async fn test_list_accounts() {
        let agent = build_playground_api_agent().await.unwrap();

        let result = agent
            .call(PlaygroundApiRequest {
                uri: "/accounts".to_string(),
                http_method: "GET".to_string(),
                ..Default::default()
            })
            .await
            .unwrap();

        let accounts: Vec<Account> = from_str(&result.payload).unwrap();
        assert_eq!(result.status, 200);
        assert!(accounts.len() > 0);
    }
}
