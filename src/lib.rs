use std::net::SocketAddr;

pub mod app_state;
pub mod database;
pub mod queries;
pub mod routes;
pub mod utilities;

use app_state::AppState;
use routes::build_routes;

pub async fn run(state: AppState) -> Result<(), ()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], state.port));

    let app = build_routes(state);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
