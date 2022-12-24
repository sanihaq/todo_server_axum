use super::helpers::spawn_app;

#[tokio::test]
async fn health_check_works() {
    let state = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}:{}/health", state.uri, state.port))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}
