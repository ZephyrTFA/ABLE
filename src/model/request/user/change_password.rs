use serde::Deserialize;

#[derive(Deserialize)]
pub struct ChangeUserPasswordRequest {
    pub user: u64,
    pub old_password: String,
    pub new_password: String,
}
