use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub user: u64,
    pub enabled: bool,
}
