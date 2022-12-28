use crate::helpers::TEST_USER;

use super::helpers::{drop_database_after_test, spawn_app};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use todo_server_axum::database::users::{self, Entity as Users};
use todo_server_axum::routes::users::{RequestCreateUser, ResponseUser};
use todo_server_axum::utilities::hash::verify_password;

#[tokio::test]
async fn create_user_works() {
    let (state, db_info) = spawn_app().await;
    let client = reqwest::Client::new();

    let req_user = RequestCreateUser {
        username: TEST_USER.username.into_owned(),
        password: TEST_USER.password.into_owned(),
    };

    let response = client
        .post(&format!("{}:{}/api/v1/users", state.uri, state.port))
        .json(&req_user)
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
        .filter(users::Column::Username.eq(req_user.username.clone()))
        .one(&state.db)
        .await
    {
        Ok(u) => match u {
            Some(user) => {
                // check against database and request
                assert_eq!(
                    user.id, 1,
                    "On database id shouldn't be {}, expected 1. port: {}, db: {}",
                    user.id, state.port, db_info.name
                );
                assert_eq!(
                    user.username, req_user.username,
                    "On database username should be {}, but it's {}. port: {}, db: {}",
                    req_user.username, user.username, state.port, db_info.name
                );
                let is_verified = verify_password(&req_user.password, &user.password)
                    .expect("error hashing password.");
                assert!(
                    is_verified,
                    "Verification failed on password from database. port: {}, db: {}",
                    state.port, db_info.name
                );
                match response.json::<ResponseUser>().await {
                    // check against Response and database
                    Ok(data) => {
                        assert_eq!(
                            data.id, 1,
                            "Received id {} as response, should be 1. port: {}, db: {}",
                            data.id, state.port, db_info.name
                        );
                        assert_eq!(
                            data.username, req_user.username,
                            "Received username {} as response, expected {}. port: {}, db: {}",
                            data.username, user.username, state.port, db_info.name
                        );
                        match user.token {
                            Some(token) => assert_eq!(
                                data.token, token,
                                "token should be the same. on database {}, received {}. port: {}, db: {}",
                                data.username, token, state.port, db_info.name
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
            }
            None => panic!(
                "username with {}, not found in database: {}, port: {}",
                req_user.username, db_info.name, state.port
            ),
        },
        Err(e) => {
            panic!(
                "Problem finding user on db. DbErr occurred , {}. port: {}, db: {}",
                e, state.port, db_info.name
            );
        }
    }

    drop_database_after_test(state.db, db_info).await;
}
