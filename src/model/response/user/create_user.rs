use serde::Serialize;

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub user_id: u64,
}
