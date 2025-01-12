use axum::Json;
use chrono::{DateTime, Utc};
use log::warn;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::model::response::api::{ApiError, ApiErrorCode, ApiResponse};

use super::permissions::{self, Permission};

pub type User = Model;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username: String,
    pub salt: String,
    pub hash: String,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub token: String,
    pub token_expiry: DateTime<Utc>,
    pub permission_id: i32,
}

#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub async fn permissions(
        &self,
        db: DatabaseConnection,
    ) -> Result<permissions::Model, Json<ApiResponse<ApiError>>> {
        let permissions = permissions::Entity::find_by_id((self.permission_id, self.id))
            .one(&db)
            .await;
        if let Err(error) = &permissions {
            warn!("Failed to fetch permission set: {error}");
            return Err(Json(ApiResponse::error(ApiError::new(
                ApiErrorCode::InternalServerError,
                String::default(),
            ))));
        }

        let permissions = permissions.unwrap();
        if permissions.is_none() {
            warn!(
                "failed to locate expected permission set! {} -> !{}!",
                self.id, self.permission_id
            );
            return Err(Json(ApiResponse::error(ApiError::new(
                ApiErrorCode::InternalServerError,
                String::default(),
            ))));
        }

        Ok(permissions.unwrap())
    }

    pub async fn assert_permission(
        &self,
        database: DatabaseConnection,
        permission: Permission,
    ) -> Result<(), Json<ApiResponse<ApiError>>> {
        if !(self.permissions(database).await? & permission) {
            return Err(Json(ApiResponse::error(ApiError::new(
                ApiErrorCode::Forbidden,
                "User lacks the permissions required to perform this action.".to_string(),
            ))));
        }
        Ok(())
    }
}
