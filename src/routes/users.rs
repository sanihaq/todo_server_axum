use serde::{Deserialize, Serialize};

pub mod create_user;
pub mod login;

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseUser {
    pub id: i32,
    pub username: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestCreateUser {
    pub username: String,
    pub password: String,
}
