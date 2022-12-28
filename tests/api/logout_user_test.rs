use crate::helpers::{setup_user, TEST_USER};

use super::helpers::{drop_database_after_test, spawn_app};
use todo_server_axum::queries::user_queries::find_by_username;

#[tokio::test]
async fn logout_user_works() {
    let (state, db_info) = spawn_app().await;
    let client = reqwest::Client::new();

    let (_request_user, _user, token) = setup_user(&state, &db_info).await;

    let response = client
        .post(&format!("{}:{}/api/v1/users/logout", state.uri, state.port))
        .header("x-auth-token", token)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(
        response.status().is_success(),
        "status code was: {}, expected code was 200. port: {}, db: {}",
        response.status(),
        state.port,
        db_info.name
    );

    let user = find_by_username(&state.db, TEST_USER.username.into_owned())
        .await
        .expect("Couldn't found user in database");

    assert_eq!(
        user.id, 1,
        "user-id should not be {}. port: {}, db: {}",
        user.id, state.port, db_info.name
    );

    if let Some(token) = user.token {
        panic!(
            "token should be empty, got {}. port: {}, db: {}",
            token, state.port, db_info.name
        )
    };

    drop_database_after_test(state.db, db_info).await;
}

#[tokio::test]
async fn logout_with_wrong_token_should_fail() {
    let (state, db_info) = spawn_app().await;
    let client = reqwest::Client::new();

    let (_request_user, _user, _token) = setup_user(&state, &db_info).await;

    let random_token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2NzIxMzc5MzMsInVzZXJuYW1lIjoiaGVsbG8ifQ.ql1lggnW7geqse5cL8wHH_7Lk4sns0BB1Q0n67ljpfk";

    let response = client
        .post(&format!("{}:{}/api/v1/users/logout", state.uri, state.port))
        .header("x-auth-token", random_token)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(
        !response.status().is_success(),
        "status code was: {}, expected code was 401. port: {}, db: {}",
        response.status(),
        state.port,
        db_info.name
    );
}

#[tokio::test]
async fn logout_with_no_token_should_fail() {
    let (state, db_info) = spawn_app().await;
    let client = reqwest::Client::new();

    let (_request_user, _user, _token) = setup_user(&state, &db_info).await;

    let response = client
        .post(&format!("{}:{}/api/v1/users/logout", state.uri, state.port))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(
        !response.status().is_success(),
        "status code was: {}, expected code was 401. port: {}, db: {}",
        response.status(),
        state.port,
        db_info.name
    );
}
