use super::helpers::spwan_app;
use reqwest::StatusCode;
use todo_server_axum::routes::users::{RequestCreateUser, ResponseUser};

#[tokio::test]
async fn create_user_works() {
    let state = spwan_app().await;
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
            assert_eq!(data.username, user.username);
        }
        Err(e) => panic!("Error decoding response: , {}", e),
    }

    let response = client
        .post(&format!("{}:{}/api/v1/users", state.uri, state.port))
        .json(&user)
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(response.status(), StatusCode::CONFLICT);
}
