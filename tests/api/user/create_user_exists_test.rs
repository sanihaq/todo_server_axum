use crate::helpers::{drop_database_after_test, setup_user, spawn_app};
use reqwest::StatusCode;

#[tokio::test]
async fn create_user_exist_works() {
    let (state, db_info) = spawn_app().await;
    let client = reqwest::Client::new();

    let (request_user, _user, _) = setup_user(&state, &db_info).await;

    let response = client
        .post(&format!("{}:{}/api/v1/users", state.uri, state.port))
        .json(&request_user)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(
        response.status(),
        StatusCode::CONFLICT,
        "expected status code was 409(CONFLICT), got {}. port: {}, db: {}",
        response.status(),
        state.port,
        db_info.name
    );

    drop_database_after_test(state.db, db_info).await;
}
