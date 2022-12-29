use reqwest::StatusCode;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use todo_server_axum::database::tasks::{self, Entity as Tasks};
use todo_server_axum::routes::tasks::{RequestTask, ResponseTask};

use crate::helpers::{drop_database_after_test, setup_user, spawn_app, TEST_TASK};

#[tokio::test]
async fn create_task_works() {
    let (state, db_info) = spawn_app().await;
    let client = reqwest::Client::new();

    let (_request_user, user, token) = setup_user(&state, &db_info).await;

    let request_task = RequestTask {
        title: Some(TEST_TASK.title.into_owned()),
        priority: None,
        description: None,
        completed_at: None,
    };

    let response = client
        .post(&format!("{}:{}/api/v1/tasks", state.uri, state.port))
        .header("x-auth-token", token)
        .json(&request_task)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(
        response.status(),
        StatusCode::CREATED,
        "status code was: {}, expected code was 201. port: {}, db: {}",
        response.status(),
        state.port,
        db_info.name,
    );

    match Tasks::find()
        .filter(tasks::Column::Title.eq(request_task.title.clone()))
        .one(&state.db)
        .await
    {
        Ok(t) => match t {
            Some(task) => {
                // check against database and request
                assert_eq!(
                    task.id, 1,
                    "On database id shouldn't be {}, expected 1. port: {}, db: {}",
                    task.id, state.port, db_info.name
                );
                assert_eq!(
                    task.user_id,
                    Some(user.id),
                    "On database id shouldn't be {}, expected 1. port: {}, db: {}",
                    task.id,
                    state.port,
                    db_info.name
                );
                assert_eq!(
                    task.title,
                    TEST_TASK.title.into_owned(),
                    "On database username should be {}, but it's {}. port: {}, db: {}",
                    TEST_TASK.title.into_owned(),
                    user.username,
                    state.port,
                    db_info.name
                );

                match response.json::<ResponseTask>().await {
                    // check against Response and database
                    Ok(data) => {
                        assert_eq!(
                            data.id, 1,
                            "Received task id {} as response, should be 1. port: {}, db: {}",
                            data.id, state.port, db_info.name
                        );
                        assert_eq!(
                            data.title, task.title,
                            "Received task title {} as response, expected {}. port: {}, db: {}",
                            data.title, task.title, state.port, db_info.name
                        );
                        assert_eq!(
                            data.priority, None,
                            "On Response task should be None. port: {}, db: {}",
                            state.port, db_info.name
                        );
                        assert_eq!(
                            data.description, None,
                            "On Response task should be None. port: {}, db: {}",
                            state.port, db_info.name
                        );
                        assert_eq!(
                            data.completed_at, None,
                            "On Response Completed at should be None. port: {}, db: {}",
                            state.port, db_info.name
                        );
                    }
                    Err(e) => {
                        panic!("Error decoding response: , {}", e);
                    }
                }
            }
            None => panic!(
                "task title with {}, not found in database: {}, port: {}",
                TEST_TASK.title.into_owned(),
                db_info.name,
                state.port
            ),
        },
        Err(e) => {
            panic!(
                "Problem finding task on db. DbErr occurred , {}. port: {}, db: {}",
                e, state.port, db_info.name
            );
        }
    }

    drop_database_after_test(state.db, db_info).await;
}

#[tokio::test]
async fn create_task_with_no_title_should_fail() {
    let (state, db_info) = spawn_app().await;
    let client = reqwest::Client::new();

    let (_request_user, _user, token) = setup_user(&state, &db_info).await;

    let request_task = RequestTask {
        title: None,
        priority: None,
        description: None,
        completed_at: None,
    };

    let response = client
        .post(&format!("{}:{}/api/v1/tasks", state.uri, state.port))
        .header("x-auth-token", token)
        .json(&request_task)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(
        response.status(),
        StatusCode::BAD_REQUEST,
        "status code was: {}, expected code was 400. port: {}, db: {}",
        response.status(),
        state.port,
        db_info.name,
    );

    drop_database_after_test(state.db, db_info).await;
}

#[tokio::test]
async fn create_task_with_no_token_should_fail() {
    let (state, db_info) = spawn_app().await;
    let client = reqwest::Client::new();

    let (_request_user, _user, _) = setup_user(&state, &db_info).await;

    let request_task = RequestTask {
        title: Some(TEST_TASK.title.into_owned()),
        priority: None,
        description: None,
        completed_at: None,
    };

    let response = client
        .post(&format!("{}:{}/api/v1/tasks", state.uri, state.port))
        // .header("x-auth-token", token)
        .json(&request_task)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(
        response.status(),
        StatusCode::UNAUTHORIZED,
        "status code was: {}, expected code was 401. port: {}, db: {}",
        response.status(),
        state.port,
        db_info.name,
    );

    drop_database_after_test(state.db, db_info).await;
}
