use serde::{Deserialize, Serialize};

pub mod create_user;

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseUser {
    pub id: i32,
    pub username: String,
    pub token: String,
}
