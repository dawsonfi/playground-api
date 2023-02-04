mod agent;

#[cfg(feature = "integration")]
#[cfg(test)]
mod tests {
    use crate::agent::{build_playground_api_agent, PlaygroundApiRequest};

    #[tokio::test]
    async fn test_list_accounts() {
        let agent = build_playground_api_agent().await.unwrap();

        let result = agent.call(PlaygroundApiRequest {
            uri: "/accounts".to_string(),
            http_method: "GET".to_string(),
            ..Default::default()
        }).await.unwrap();

        assert_eq!(result.status, 200);
        assert_eq!(result.payload, "{}".to_string());
    }
}
