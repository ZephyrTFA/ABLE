use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginResponse {
    pub token: String,
    pub token_expiry: DateTime<Utc>,
}
