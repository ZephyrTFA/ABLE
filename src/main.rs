use std::env;

use ::log::{error, info, warn};
use config::Config;
use dotenv::dotenv;
use library::Library;
use routes::init_router;
use tokio::net::TcpListener;

pub mod config;
pub mod database;
pub mod library;
pub mod model;
pub mod orm;
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

    let database_connection_string = env::var("DATABASE_URL")
        .unwrap_or("mysql://library:library@localhost:3306/library".to_string());
    let library = Library::new(&database_connection_string).await;
    if let Err(error) = &library {
        warn!("Failed to initialize library: {error}");
        return;
    }
    let app = init_router(library.unwrap());

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
