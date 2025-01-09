use axum::{extract::State, http::HeaderMap, routing::post, Form, Json, Router};
use axum_login::tracing::warn;
use chrono::Utc;
use log::debug;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    auth::{UserAuthentication, UserAuthenticationError},
    model::{
        request::login::LoginRequest,
        response::{
            api::{ApiError, ApiErrorCode, ApiResponse},
            login::LoginResponse,
        },
    },
    orm::user,
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

pub async fn login_from_headers(
    db: &DatabaseConnection,
    headers: &HeaderMap,
) -> Result<user::Model, Json<ApiResponse<ApiError>>> {
    let auth_header = headers.get("authorization");
    if auth_header.is_none() {
        return Err(Json(ApiResponse::error(ApiError::new(
            ApiErrorCode::Unauthorized,
            "No authorization token passed.".to_string(),
        ))));
    }
    let token = auth_header.unwrap().to_str();
    if token.is_err() {
        return Err(Json(ApiResponse::error(ApiError::new(
            ApiErrorCode::BadRequest,
            "Malformed authorization token.".to_string(),
        ))));
    }

    return login_from_token(db, token.unwrap()).await;
}

pub async fn login_from_token(
    db: &DatabaseConnection,
    token: &str,
) -> Result<user::Model, Json<ApiResponse<ApiError>>> {
    let db_result = user::Entity::find()
        .filter(user::Column::Token.eq(token))
        .filter(user::Column::TokenExpiry.gt(Utc::now()))
        .one(db)
        .await;
    if let Err(error) = &db_result {
        warn!("Failed to query db for token authentication: {error}");
        return Err(Json(ApiResponse::error(ApiError::new(
            ApiErrorCode::InternalServerError,
            String::new(),
        ))));
    }

    let user = db_result.unwrap();
    if user.is_none() {
        return Err(Json(ApiResponse::error(ApiError::new(
            ApiErrorCode::Unauthorized,
            String::new(),
        ))));
    }
    Ok(user.unwrap())
}
