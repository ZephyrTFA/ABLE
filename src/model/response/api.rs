use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum ApiErrorCode {
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    InternalServerError = 500,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiResponse<T: Serialize + Send> {
    status: String,
    data: T,
}

impl<T: Serialize + Send> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            status: "success".to_string(),
            data,
        }
    }

    pub fn error(data: T) -> Self {
        Self {
            status: "error".to_string(),
            data,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
