use super::helpers::spwan_app;
use todo_server_axum::routes::users::ResponseUser;

#[tokio::test]
async fn create_user_works() {
    let state = spwan_app().await;
    let client = reqwest::Client::new();

    let _mock_user = ResponseUser {
        id: 123,
        username: "".to_owned(),
        token: "".to_owned(),
    };

    let response = client
        .post(&format!(
            "{}:{}/api/v1/users",
            "http://localhost", state.port
        ))
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
