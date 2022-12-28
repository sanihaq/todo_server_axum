use crate::helpers::setup_user;

use super::helpers::{drop_database_after_test, spawn_app};
use todo_server_axum::queries::user_queries::find_by_username;
use todo_server_axum::routes::users::ResponseUser;

#[tokio::test]
async fn login_user_works() {
    let (state, db_info) = spawn_app().await;
    let client = reqwest::Client::new();
    let (request_user, _user, _) = setup_user(&state, &db_info).await;

    let response = client
        .post(&format!("{}:{}/api/v1/users/login", state.uri, state.port))
        .json(&request_user)
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

    let user = find_by_username(&state.db, request_user.username)
        .await
        .expect("Couldn't found user in database");

    match response.json::<ResponseUser>().await {
        Ok(data) => {
            assert_eq!(
                data.id, 1,
                "user-id should not be {}. port: {}, db: {}",
                data.id, state.port, db_info.name
            );
            assert_eq!(
                data.username, user.username,
                "expected username was {}, got {}. port: {}, db: {}",
                data.username, user.username, state.port, db_info.name
            );
            match user.token {
                Some(token) => assert_eq!(
                    data.token, token,
                    "expected username was {}, got {}. port: {}, db: {}",
                    data.username, user.username, state.port, db_info.name
                ),
                None => panic!(
                    "token not found in database: {}, port: {}",
                    db_info.name, state.port
                ),
            };
        }
        Err(e) => {
            panic!("Error decoding response: , {}", e);
        }
    }

    drop_database_after_test(state.db, db_info).await;
}

#[tokio::test]
async fn login_user_with_wrong_username_should_fail() {
    let (state, db_info) = spawn_app().await;
    let client = reqwest::Client::new();
    let (mut request_user, _user, _) = setup_user(&state, &db_info).await;

    request_user.username = "username".to_string();

    let response = client
        .post(&format!("{}:{}/api/v1/users/login", state.uri, state.port))
        .json(&request_user)
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(
        !response.status().is_success(),
        "status code was: {}, expected code was 400. port: {}, db: {}",
        response.status(),
        state.port,
        db_info.name
    );
}

#[tokio::test]
async fn login_user_with_wrong_password_should_fail() {
    let (state, db_info) = spawn_app().await;
    let client = reqwest::Client::new();
    let (mut request_user, _user, _) = setup_user(&state, &db_info).await;

    request_user.username = "password".to_string();

    let response = client
        .post(&format!("{}:{}/api/v1/users/login", state.uri, state.port))
        .json(&request_user)
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(
        !response.status().is_success(),
        "status code was: {}, expected code was 400. port: {}, db: {}",
        response.status(),
        state.port,
        db_info.name
    );
}
