use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ApiErrorCode {
    InternalServerError = 255,
}

#[derive(Serialize, Debug)]
pub struct ApiResponse<T: Serialize> {
    status: String,
    #[serde(flatten)]
    data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: Option<T>) -> Self {
        Self {
            status: "success".to_string(),
            data,
        }
    }

    pub fn error(data: T) -> Self {
        Self {
            status: "error".to_string(),
            data: Some(data),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct ApiError {
    error: ApiErrorCode,
    message: String,
}

impl ApiError {
    pub fn new(error: ApiErrorCode, message: String) -> Self {
        Self { error, message }
    }
}
