use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::{TimeDelta, Utc};
use log::warn;
use rand::{distributions::Alphanumeric, Rng};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter};

use crate::{
    model::{request::login::LoginRequest, response::login::LoginResponse},
    orm::user::{self, User},
};

pub struct UserAuthentication;

#[derive(Clone, Debug)]
pub enum UserAuthenticationError {
    InternalServerError,
    Unauthorized,
}

impl UserAuthentication {
    pub async fn try_login(
        login: LoginRequest,
        db: DatabaseConnection,
    ) -> Result<LoginResponse, UserAuthenticationError> {
        let user = user::Entity::find()
            .filter(user::Column::Username.eq(login.username))
            .one(&db)
            .await;
        if let Err(error) = &user {
            warn!("Failed to login user: {error}");
            return Err(UserAuthenticationError::InternalServerError);
        }

        let user = user.unwrap();
        if user.is_none() {
            return Err(UserAuthenticationError::Unauthorized);
        }
        let user = user.unwrap();

        let argon2 = Argon2::default();
        let password =
            PasswordHash::new(&user.hash).expect("failed to convert db hash to runtime hash");
        if argon2
            .verify_password(login.password.as_bytes(), &password)
            .is_err()
        {
            return Err(UserAuthenticationError::Unauthorized);
        }

        let token = format!(
            "token_able{}",
            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(32)
                .map(char::from)
                .collect::<String>()
        );
        let token_expiry = Utc::now()
            .checked_add_signed(TimeDelta::hours(2))
            .expect("failed to add 2 hours to a datetime???");

        if let Err(error) = user::Entity::update(
            User {
                token: token.clone(),
                token_expiry,
                ..user
            }
            .into_active_model(),
        )
        .exec(&db)
        .await
        {
            warn!("Failed to update db with user token: {error}");
            return Err(UserAuthenticationError::InternalServerError);
        }

        Ok(LoginResponse {
            token,
            token_expiry,
        })
    }
}
