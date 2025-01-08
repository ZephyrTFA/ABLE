use std::{
    env,
    sync::Arc,
    thread::{sleep, spawn},
    time::Duration,
};

use ::log::{error, info, warn};
use config::Config;
use dotenv::dotenv;
use routes::init_router;
use sea_orm::Database;
use state::create_state;
use tokio::net::TcpListener;
use tower_governor::{
    governor::GovernorConfigBuilder, key_extractor::SmartIpKeyExtractor, GovernorLayer,
};

pub mod auth;
pub mod config;
pub mod database;
pub mod library;
pub mod model;
pub mod orm;
pub mod routes;
pub mod state;

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
    let connection = Database::connect(&database_connection_string).await;
    if let Err(error) = &connection {
        error!("Failed to initialize db connection: {error}");
        return;
    }
    let connection = connection.unwrap();
    let state = create_state(connection);
    let app = init_router(state);
    let governor_config = Arc::new(
        GovernorConfigBuilder::default()
            .key_extractor(SmartIpKeyExtractor)
            .per_second(config.rate_limit_per_second())
            .burst_size(config.rate_limit_burst())
            .finish()
            .unwrap(),
    );
    let governor = governor_config.limiter().clone();
    let interval = Duration::from_secs(60);
    spawn(move || loop {
        sleep(interval);
        governor.retain_recent();
    });
    let app = app.layer(GovernorLayer {
        config: governor_config,
    });

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
