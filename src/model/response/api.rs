use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ApiErrorCode {
    InternalServerError = 255,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ApiResponse {
    Error(ApiErrorResponse),
    Success(),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiErrorResponse {
    status: String,
    code: ApiErrorCode,
    message: Option<String>,
}

impl ApiErrorResponse {
    pub fn new(error: ApiErrorCode, message: Option<String>) -> Self {
        Self {
            status: "error".to_string(),
            code: error,
            message,
        }
    }
}
