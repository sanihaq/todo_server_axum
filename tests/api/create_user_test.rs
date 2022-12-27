use crate::helpers::TEST_USER;

use super::helpers::{drop_database_after_test, spawn_app};
use reqwest::StatusCode;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use todo_server_axum::database::users::{self, Entity as Users};
use todo_server_axum::routes::users::{RequestCreateUser, ResponseUser};

#[tokio::test]
async fn create_user_works() {
    let (state, db_info) = spawn_app().await;
    let client = reqwest::Client::new();

    let user = RequestCreateUser {
        username: TEST_USER.username.into_owned(),
        password: TEST_USER.password.into_owned(),
    };

    let response = client
        .post(&format!("{}:{}/api/v1/users", state.uri, state.port))
        .json(&user)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(
        response.status().is_success(),
        "status code was: {}, expected code was 200. port: {}, db: {}",
        response.status(),
        state.port,
        db_info.name,
    );

    match Users::find()
        .filter(users::Column::Username.eq(user.username.clone()))
        .one(&state.db)
        .await
    {
        Ok(u) => match u {
            Some(user) => assert_eq!(
                user.id, 1,
                "On database id shouldn't be {}. port: {}, db: {}",
                user.id, state.port, db_info.name
            ),
            None => panic!(
                "username with {}, not found in database: {}, port: {}",
                user.username, db_info.name, state.port
            ),
        },
        Err(e) => {
            eprintln!(
                "Problem finding user on db. DbErr occurred , {}. port: {}, db: {}",
                e, state.port, db_info.name
            );
        }
    }

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
        }
        Err(e) => {
            eprintln!("Error decoding response: , {}", e);
        }
    }

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
