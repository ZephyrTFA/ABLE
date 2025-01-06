use axum::Router;
use config::Config;
use dotenv::dotenv;
use routes::register_routes;
use tokio::net::TcpListener;

pub mod config;
pub mod model;
pub mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = Config::from_env();
    if config.is_err() {
        eprintln!("Failed to initialize config: `{}`", config.unwrap_err());
        return;
    }
    let config = config.unwrap();

    println!("Initializing router.");
    let app = register_routes(Router::new());

    let target_bind = format!("{}:{}", config.bind_address(), config.bind_port());
    println!("Initializing server at http://{target_bind}.");
    let server = TcpListener::bind(target_bind).await;
    if server.is_err() {
        eprintln!("Failed to bind TcpListener: `{}`", server.unwrap_err());
        return;
    }
    let server = server.unwrap();

    let axum = axum::serve(server, app);
    println!("Ready.");

    let result = axum.await;
    if let Err(error) = result {
        eprintln!("Unrecoverable error: `{error}`");
    } else {
        println!("Server closed.");
    }
}
