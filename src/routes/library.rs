use std::sync::{Mutex, MutexGuard};

use axum::{extract, Json};
use log::{error, trace, warn};
use once_cell::sync::Lazy;

use crate::{
    library::Library,
    model::response::{
        api::{ApiError, ApiErrorCode, ApiResponse},
        book::Book,
    },
};

type Response<T> = Result<Json<ApiResponse<T>>, Json<ApiResponse<ApiError>>>;

fn get_library() -> Result<MutexGuard<'static, Library>, ApiResponse<ApiError>> {
    trace!("getting library lock");
    static LIBRARY: Lazy<Mutex<Library>> = Lazy::new(|| Mutex::new(Library::default()));

    let library = LIBRARY.lock();
    if let Err(error) = &library {
        warn!("library mutex is poisoned");
        return Err(ApiResponse::error(ApiError::new(
            ApiErrorCode::InternalServerError,
            error.to_string(),
        )));
    }
    let library = library.unwrap();

    Ok(library)
}

pub async fn add_book(extract::Json(_book): extract::Json<Book>) -> Response<String> {
    let _library = get_library()?;

    error!("not implemented at {}:{}", file!(), line!());
    Err(Json(ApiResponse::error(ApiError::new(
        ApiErrorCode::InternalServerError,
        "not implemented".to_string(),
    ))))
}

pub async fn get_books<'a>() -> Response<String> {
    let _library = get_library()?;

    error!("not implemented at {}:{}", file!(), line!());
    Err(Json(ApiResponse::error(ApiError::new(
        ApiErrorCode::InternalServerError,
        "not implemented".to_string(),
    ))))
}

pub async fn get_book_by_id(extract::Path(_id): extract::Path<u32>) -> Response<String> {
    let _library = get_library()?;

    error!("not implemented at {}:{}", file!(), line!());
    Err(Json(ApiResponse::error(ApiError::new(
        ApiErrorCode::InternalServerError,
        "not implemented".to_string(),
    ))))
}

pub async fn update_book(
    extract::Path(_id): extract::Path<u32>,
    extract::Json(_book): extract::Json<Book>,
) -> Response<String> {
    let _library = get_library()?;

    error!("not implemented at {}:{}", file!(), line!());
    Err(Json(ApiResponse::error(ApiError::new(
        ApiErrorCode::InternalServerError,
        "not implemented".to_string(),
    ))))
}

pub async fn drop_book(extract::Path(_id): extract::Path<u32>) -> Response<String> {
    let _library = get_library()?;

    error!("not implemented at {}:{}", file!(), line!());
    Err(Json(ApiResponse::error(ApiError::new(
        ApiErrorCode::InternalServerError,
        "not implemented".to_string(),
    ))))
}
