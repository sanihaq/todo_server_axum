use crate::helpers::TEST_USER;

use super::helpers::{drop_database_after_test, spawn_app};
use sea_orm::Set;
use todo_server_axum::database::users::{self};
use todo_server_axum::queries::user_queries::{find_by_username, save_active_user};
use todo_server_axum::utilities::hash::hash_password;
use todo_server_axum::utilities::jwt::create_token;

#[tokio::test]
async fn logout_user_works() {
    let (state, db_info) = spawn_app().await;
    let client = reqwest::Client::new();

    let mut user = users::ActiveModel {
        ..Default::default()
    };

    user.username = Set(TEST_USER.username.into_owned());
    user.password = Set(hash_password(&TEST_USER.password).expect("error hashing password."));

    let token = create_token(&state.jwt_secret.0, TEST_USER.username.into_owned())
        .expect("error creating token.");

    user.token = Set(Some(token.clone()));

    let _ = save_active_user(&state.db, user).await.unwrap_or_else(|_| {
        panic!(
            "Unable to save in database.  port: {}, db: {}",
            state.port, db_info.name
        )
    });

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
