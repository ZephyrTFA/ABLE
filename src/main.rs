use ::log::{error, info, warn};
use axum::Router;
use config::Config;
use dotenv::dotenv;
use routes::register_routes;
use tokio::net::TcpListener;

pub mod config;
pub mod library;
pub mod model;
pub mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    #[cfg(debug_assertions)]
    {
        env_logger::builder()
            .filter_level(::log::LevelFilter::Trace)
            .init();
        warn!("Logging overriden to show all due to debug environment.");
    }

    #[cfg(not(debug_assertions))]
    env_logger::init();

    let config = Config::from_env();
    if config.is_err() {
        error!("Failed to initialize config: `{}`", config.unwrap_err());
        return;
    }
    let config = config.unwrap();

    info!("Initializing router.");
    let app = register_routes(Router::new());

    let target_bind = format!("{}:{}", config.bind_address(), config.bind_port());
    info!("Initializing server at http://{target_bind}.");
    let server = TcpListener::bind(target_bind).await;
    if server.is_err() {
        error!("Failed to bind TcpListener: `{}`", server.unwrap_err());
        return;
    }
    let server = server.unwrap();

    let axum = axum::serve(server, app);
    info!("Ready.");

    let result = axum.await;
    if let Err(error) = result {
        error!("Unrecoverable error: `{error}`");
    } else {
        info!("Server closed.");
    }
}
