use axum::Json;

use super::ResponseUser;

pub async fn create_user() -> Result<Json<ResponseUser>, ()> {
    Ok(Json(ResponseUser {
        id: 1,
        username: "".to_owned(),
        token: "".to_owned(),
    }))
}
