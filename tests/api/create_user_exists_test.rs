use crate::helpers::TEST_USER;

use super::helpers::{drop_database_after_test, spawn_app};
use reqwest::StatusCode;
use sea_orm::Set;
use todo_server_axum::database::users::{self};
use todo_server_axum::queries::user_queries::save_active_user;
use todo_server_axum::routes::users::RequestCreateUser;
use todo_server_axum::utilities::hash::hash_password;

#[tokio::test]
async fn create_user_exist_works() {
    let (state, db_info) = spawn_app().await;
    let client = reqwest::Client::new();

    let mut user = users::ActiveModel {
        ..Default::default()
    };

    user.username = Set(TEST_USER.username.into_owned());
    user.password =
        Set(hash_password(&TEST_USER.password.into_owned()).expect("error hashing password."));

    let user = save_active_user(&state.db, user).await.expect(
        format!(
            "Unable to save in database.  port: {}, db: {}",
            state.port, db_info.name
        )
        .as_str(),
    );

    let user = RequestCreateUser {
        username: user.username,
        password: user.password,
    };

    let response = client
        .post(&format!("{}:{}/api/v1/users", state.uri, state.port))
        .json(&user)
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
