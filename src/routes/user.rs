use std::sync::Arc;

use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{
    extract::{Path, State},
    routing::put,
    Json, Router,
};
use axum_login::tracing::warn;
use chrono::Utc;
use password_hash::SaltString;
use rand::thread_rng;
use sea_orm::{ActiveValue, EntityTrait, IntoActiveModel, Set};
use tokio::sync::Mutex;

use crate::{
    model::{
        request::user::{
            change_password::ChangeUserPasswordRequest, create_user::CreateUserRequest,
            update_user::UpdateUserRequest,
        },
        response::{
            api::{ApiError, ApiErrorCode, ApiResponse},
            user::{
                change_password::ChangeUserPasswordResponse, create_user::CreateUserResponse,
                update_user::UpdateUserResponse,
            },
        },
    },
    orm::{
        permissions::{self, Permission},
        user,
    },
    state::AppState,
};

use super::{login::ApiUser, Response};

pub fn user_router() -> Router<Arc<Mutex<AppState>>> {
    Router::new()
        .route("/", put(create_user))
        .route("/{id}", put(update_user))
        .route("/password/{id}", put(change_password))
}

async fn change_password(
    State(state): State<Arc<Mutex<AppState>>>,
    ApiUser(caller): ApiUser,
    Json(change_password_request): Json<ChangeUserPasswordRequest>,
) -> Response<ChangeUserPasswordResponse> {
    let state = state.lock().await;

    let change_own_password = caller.id == change_password_request.user;

    if !change_own_password {
        caller
            .assert_permission(state.db(), Permission::UserUpdate)
            .await?;
    }

    let target = if change_own_password {
        caller
    } else {
        let db_result = user::Entity::find_by_id(change_password_request.user)
            .one(&state.db())
            .await;
        if let Err(error) = &db_result {
            warn!("failed to query db: {error}");
            return Err(Json(ApiResponse::error(ApiError::new(
                ApiErrorCode::InternalServerError,
                String::new(),
            ))));
        }

        let target = db_result.unwrap();
        if target.is_none() {
            return Err(Json(ApiResponse::error(ApiError::new(
                ApiErrorCode::NotFound,
                String::new(),
            ))));
        }
        target.unwrap()
    };

    let argon = Argon2::default();
    if !change_own_password {
        let old_hash = PasswordHash::new(&target.hash);
        if let Err(error) = &old_hash {
            warn!("illegal hash in db! {error}");
            return Err(Json(ApiResponse::error(ApiError::new(
                ApiErrorCode::InternalServerError,
                String::new(),
            ))));
        }
        if argon
            .verify_password(
                change_password_request.old_password.as_bytes(),
                old_hash.as_ref().unwrap(),
            )
            .is_err()
        {
            return Err(Json(ApiResponse::error(ApiError::new(
                ApiErrorCode::BadRequest,
                "passwords did not match".to_string(),
            ))));
        }
    }

    let salt = SaltString::from_b64(&target.salt);
    if let Err(error) = &salt {
        warn!("failed to construct Salt from db: {error}");
        return Err(Json(ApiResponse::error(ApiError::new(
            ApiErrorCode::InternalServerError,
            String::new(),
        ))));
    }
    let salt = salt.unwrap();

    let new_hash = argon.hash_password(
        change_password_request.new_password.as_bytes(),
        salt.as_salt(),
    );
    if let Err(error) = new_hash {
        warn!("failed to hash password: {error}");
        return Err(Json(ApiResponse::error(ApiError::new(
            ApiErrorCode::InternalServerError,
            "failed to hash new password".to_string(),
        ))));
    }

    let updated_user = user::ActiveModel {
        hash: Set(new_hash.unwrap().to_string()),
        ..target.into_active_model()
    };
    let db_result = user::Entity::update(updated_user).exec(&state.db()).await;
    if let Err(error) = db_result {
        warn!("failed to update password: {error}");
        return Err(Json(ApiResponse::error(ApiError::new(
            ApiErrorCode::InternalServerError,
            "failed to update password".to_string(),
        ))));
    }

    Ok(Json(ApiResponse::success(ChangeUserPasswordResponse)))
}

async fn update_user(
    State(state): State<Arc<Mutex<AppState>>>,
    ApiUser(caller): ApiUser,
    Path(target_id): Path<u64>,
    Json(update_user_request): Json<UpdateUserRequest>,
) -> Response<UpdateUserResponse> {
    let state = state.lock().await;

    caller
        .assert_permission(state.db(), Permission::UserUpdate)
        .await?;

    if target_id != update_user_request.user {
        return Err(Json(ApiResponse::error(ApiError::new(
            ApiErrorCode::BadRequest,
            "target id does not match request body".to_string(),
        ))));
    }

    let user_active = user::ActiveModel {
        enabled: Set(update_user_request.enabled),
        ..Default::default()
    };
    let db_result = user::Entity::update(user_active).exec(&state.db()).await;
    if let Err(error) = db_result {
        warn!("failed to update user: {error}");
        return Err(Json(ApiResponse::error(ApiError::new(
            ApiErrorCode::InternalServerError,
            "failed to update db".to_string(),
        ))));
    }

    Ok(Json(ApiResponse::success(UpdateUserResponse)))
}

async fn create_user(
    State(state): State<Arc<Mutex<AppState>>>,
    ApiUser(caller): ApiUser,
    Json(create_user_request): Json<CreateUserRequest>,
) -> Response<CreateUserResponse> {
    let state = state.lock().await;

    caller
        .assert_permission(state.db(), Permission::UserAdd)
        .await?;

    let argon = Argon2::default();
    let salt = SaltString::generate(thread_rng());

    let hashed = argon.hash_password(create_user_request.password.as_bytes(), salt.as_salt());
    if let Err(error) = &hashed {
        warn!("Failed to hash the password for a user creation request: {error}");
        return Err(Json(ApiResponse::error(ApiError::new(
            ApiErrorCode::InternalServerError,
            "failed to hash password".to_string(),
        ))));
    }
    let hashed = hashed.unwrap();

    let db_result = user::Entity::insert(user::ActiveModel {
        username: ActiveValue::Set(create_user_request.username),
        salt: ActiveValue::Set(salt.as_str().to_string()),
        enabled: ActiveValue::Set(false),
        created_at: ActiveValue::Set(Utc::now()),
        hash: ActiveValue::Set(hashed.to_string()),
        ..user::ActiveModel::default()
    })
    .exec_with_returning(&state.db())
    .await;
    if let Err(error) = &db_result {
        warn!("Failed to create new user row: {error}");
        return Err(Json(ApiResponse::error(ApiError::new(
            ApiErrorCode::InternalServerError,
            "failed to create user".to_string(),
        ))));
    }
    let mut new_user = db_result.unwrap().into_active_model();

    let db_result = permissions::Entity::insert(permissions::ActiveModel {
        user: new_user.to_owned().id,
        ..Default::default()
    })
    .exec_with_returning(&state.db())
    .await;
    if let Err(error) = &db_result {
        warn!("Failed to create new permissions row: {error}");
        return Err(Json(ApiResponse::error(ApiError::new(
            ApiErrorCode::InternalServerError,
            "failed to create user".to_string(),
        ))));
    }

    let permissions = db_result.unwrap();
    new_user.permission_id = Set(permissions.id);
    if let Err(error) = user::Entity::update(new_user.to_owned())
        .exec(&state.db())
        .await
    {
        warn!("Failed to assign user permission id: {error}");
        return Err(Json(ApiResponse::error(ApiError::new(
            ApiErrorCode::InternalServerError,
            "failed to create user".to_string(),
        ))));
    }

    Ok(Json(ApiResponse::success(CreateUserResponse {
        user_id: new_user.id.unwrap() as u64,
    })))
}
