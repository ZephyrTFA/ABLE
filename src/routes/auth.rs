use axum::{
    extract::{Path, State},
    routing::{get, put},
    Json, Router,
};
use axum_login::tracing::warn;
use sea_orm::{EntityTrait, IntoActiveModel, QueryFilter};

use crate::{
    model::response::{
        api::{ApiError, ApiErrorCode, ApiResponse},
        get_permissions::GetPermissionsResponse,
        set_permissions::SetPermissionsResponse,
    },
    orm::{
        permissions::{self, Permission},
        user,
    },
    state::AppState,
};

pub type UserPermissions = permissions::Model;
use super::{login::ApiUser, Response};

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/{id}", get(get_permissions))
        .route("/{id}", put(set_permissions))
}

async fn get_permissions(
    State(state): State<AppState>,
    ApiUser(_): ApiUser,
    Path(target_id): Path<u64>,
) -> Response<GetPermissionsResponse> {
    let db_result = user::Entity::find_by_id(target_id).one(&state.db()).await;
    if let Err(error) = &db_result {
        warn!("Failed to fetch user: {error}");
        return Err(Json(ApiResponse::error(ApiError::new(
            ApiErrorCode::InternalServerError,
            String::new(),
        ))));
    }

    let target_user = db_result.unwrap();
    if target_user.is_none() {
        return Err(Json(ApiResponse::error(ApiError::new(
            ApiErrorCode::NotFound,
            "User does not exist.".to_string(),
        ))));
    }
    let target_user = target_user.unwrap();
    let permissions = target_user.permissions(state.db()).await?;
    Ok(Json(ApiResponse::success(GetPermissionsResponse {
        permissions,
    })))
}

async fn set_permissions(
    State(state): State<AppState>,
    ApiUser(caller): ApiUser,
    Json(permissions): Json<UserPermissions>,
) -> Response<SetPermissionsResponse> {
    caller
        .assert_permission(state.db(), Permission::PermissionsUpdate)
        .await?;

    let db_result = permissions::Entity::update(permissions.clone().into_active_model())
        .belongs_to(&permissions)
        .exec(&state.db())
        .await;

    if let Err(error) = &db_result {
        warn!("Failed to update user permissions: {error}");
        return Err(Json(ApiResponse::error(ApiError::new(
            ApiErrorCode::InternalServerError,
            String::new(),
        ))));
    }

    Ok(Json(ApiResponse::success(SetPermissionsResponse)))
}
