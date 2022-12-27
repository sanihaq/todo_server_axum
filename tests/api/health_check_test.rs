use super::helpers::{drop_database_after_test, spawn_app};

#[tokio::test]
async fn health_check_works() {
    let (state, db_info) = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}:{}/health", state.uri, state.port))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(
        response.status().is_success(),
        "expected response to succeed, port: {}, db: {}",
        state.port,
        db_info.name
    );
    assert_eq!(
        response.content_length(),
        Some(0),
        "expected to receive no content.  port: {}, db: {}",
        state.port,
        db_info.name
    );

    drop_database_after_test(state.db, db_info).await;
}
