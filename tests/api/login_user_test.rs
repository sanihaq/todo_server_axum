use crate::helpers::TEST_USER;

use super::helpers::{drop_database_after_test, spawn_app};
use sea_orm::Set;
use todo_server_axum::database::users::{self};
use todo_server_axum::queries::user_queries::{find_by_username, save_active_user};
use todo_server_axum::routes::users::{RequestCreateUser, ResponseUser};
use todo_server_axum::utilities::hash::hash_password;

#[tokio::test]
async fn login_user_works() {
    let (state, db_info) = spawn_app().await;
    let client = reqwest::Client::new();

    let request_user = RequestCreateUser {
        username: TEST_USER.username.into_owned(),
        password: TEST_USER.password.into_owned(),
    };

    let mut user = users::ActiveModel {
        ..Default::default()
    };

    user.username = Set(request_user.username.clone());
    user.password = Set(hash_password(&request_user.password).expect("error hashing password."));

    let _ = save_active_user(&state.db, user).await.unwrap_or_else(|_| {
        panic!(
            "Unable to save in database.  port: {}, db: {}",
            state.port, db_info.name
        )
    });

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
            eprintln!("Error decoding response: , {}", e);
        }
    }

    drop_database_after_test(state.db, db_info).await;
}
