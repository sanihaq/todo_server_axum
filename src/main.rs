use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use todo_server_axum::{app_state::AppState, run};

#[tokio::main]
async fn main() -> Result<(), ()> {
    dotenv().ok();
    let uri: String = dotenv!("API_URI").to_owned();
    let port: String = dotenv!("API_PORT").to_owned();
    let port = port.parse::<u16>().unwrap();

    let app_state = AppState { port, uri };

    run(&app_state).await
}
