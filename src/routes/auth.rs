use axum::{extract::State, routing::post, Form, Json, Router};
use log::debug;

use crate::{
    auth::{UserAuthentication, UserAuthenticationError},
    model::{
        request::login::LoginRequest,
        response::{
            api::{ApiError, ApiErrorCode, ApiResponse},
            login::LoginResponse,
        },
    },
    state::AppState,
};

use super::Response;

pub fn auth_router() -> Router<AppState> {
    debug!("Registering authentication router.");
    Router::new().route("/login", post(login))
}

async fn login(
    State(state): State<AppState>,
    Form(login): Form<LoginRequest>,
) -> Response<LoginResponse> {
    let db = state.db();
    let login = UserAuthentication::try_login(login, db).await;

    if let Err(error) = &login {
        return Err(Json(match error {
            UserAuthenticationError::InternalServerError => ApiResponse::error(ApiError::new(
                ApiErrorCode::InternalServerError,
                String::new(),
            )),
            UserAuthenticationError::Unauthorized => {
                ApiResponse::error(ApiError::new(ApiErrorCode::Unauthorized, String::new()))
            }
        }));
    }

    Ok(Json(ApiResponse::success(Some(login.unwrap()))))
}
