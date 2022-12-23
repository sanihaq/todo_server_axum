use super::helpers::spwan_app;
use todo_server_axum::routes::users::{RequestCreateUser, ResponseUser};

#[tokio::test]
async fn create_user_works() {
    let state = spwan_app().await;
    let client = reqwest::Client::new();

    let response = client
        .post(&format!("{}:{}/api/v1/users", state.uri, state.port))
        .body(RequestCreateUser {
            username: "hello".to_owned(),
            password: "world".to_owned(),
        })
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());

    match response.json::<ResponseUser>().await {
        Ok(data) => {
            assert_eq!(data.username, "");
        }
        Err(e) => panic!("Error decoding response: , {}", e),
    }
}
