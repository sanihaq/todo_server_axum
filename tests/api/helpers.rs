use std::net::TcpListener;
use todo_server_axum::{app_state::AppState, run};

fn get_available_port() -> Option<u16> {
    (8000..9000).find(|port| port_is_available(*port))
}

fn port_is_available(port: u16) -> bool {
    match TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub async fn spwan_app() -> AppState {
    let state;
    if let Some(port) = get_available_port() {
        let app_state = AppState { port };
        state = app_state.clone();
        tokio::spawn(async move { run(&app_state).await.expect("Failed to run the server") });
    } else {
        panic!("problem finding a port!")
    }
    state
}
