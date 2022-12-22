use std::net::SocketAddr;

pub mod app_state;
pub mod routes;

use app_state::AppState;
use routes::build_routes;

pub async fn run(state: &AppState) -> Result<(), ()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], state.port));

    let app = build_routes();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
