use std::sync::Arc;

use auth::auth_router;
use axum::{Json, Router};
use library::library_router;
use log::trace;
use login::login_router;
use tokio::sync::Mutex;
use user::user_router;

use crate::{
    model::response::api::{ApiError, ApiResponse},
    state::AppState,
};

mod auth;
mod library;
mod login;
mod user;

pub type Response<T> = Result<Json<ApiResponse<T>>, Json<ApiResponse<ApiError>>>;

pub fn init_router(state: AppState) -> Router {
    trace!("Registering routes.");
    Router::new()
        .nest("/books", library_router())
        .nest("/login", login_router())
        .nest("/auth", auth_router())
        .nest("/user", user_router())
        .with_state(Arc::new(Mutex::new(state)))
}
