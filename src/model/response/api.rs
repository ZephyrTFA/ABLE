use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum ApiErrorCode {
    BadRequest = 400,
    NotFound = 404,
    InternalServerError = 500,
}

#[derive(Serialize, Debug)]
pub struct ApiResponse<T: Serialize> {
    status: String,
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
    error_code: usize,
    message: String,
}

impl ApiError {
    pub fn new(error: ApiErrorCode, message: String) -> Self {
        Self {
            error_code: error as usize,
            error,
            message,
        }
    }
}
