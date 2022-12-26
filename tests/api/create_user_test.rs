use super::helpers::{drop_database_after_test, spawn_app};
use reqwest::StatusCode;
use todo_server_axum::routes::users::{RequestCreateUser, ResponseUser};

#[tokio::test]
async fn create_user_works() {
    let (state, db_info) = spawn_app().await;
    let client = reqwest::Client::new();

    let user = RequestCreateUser {
        username: "hello".to_owned(),
        password: "world".to_owned(),
    };

    let response = client
        .post(&format!("{}:{}/api/v1/users", state.uri, state.port))
        .json(&user)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(
        response.status().is_success(),
        "status code was: {}, expected code was 200",
        response.status()
    );

    match response.json::<ResponseUser>().await {
        Ok(data) => {
            assert!(data.id > 0);
            assert_eq!(data.username, user.username);
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
    assert_eq!(response.status(), StatusCode::CONFLICT);

    drop_database_after_test(state.db, db_info).await;
}
