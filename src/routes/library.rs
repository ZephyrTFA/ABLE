use std::sync::{Mutex, MutexGuard};

use axum::{extract, Json};
use log::{error, trace, warn};
use once_cell::sync::Lazy;

use crate::{
    library::Library,
    model::response::{
        api::{ApiErrorCode, ApiErrorResponse, ApiResponse},
        book::Book,
    },
};

fn get_library() -> Result<MutexGuard<'static, Library>, ApiResponse> {
    trace!("getting library lock");
    static LIBRARY: Lazy<Mutex<Library>> = Lazy::new(|| Mutex::new(Library::default()));

    let library = LIBRARY.lock();
    if let Err(error) = &library {
        warn!("library mutex is poisoned");
        return Err(ApiResponse::Error(ApiErrorResponse::new(
            ApiErrorCode::InternalServerError,
            Some(error.to_string()),
        )));
    }
    let library = library.unwrap();

    Ok(library)
}

pub async fn add_book(extract::Json(book): extract::Json<Book>) -> Json<ApiResponse> {
    let library = get_library();
    if let Err(err) = library {
        return Json(err);
    }
    let mut library = library.unwrap();

    let result = library.add_book(book);
    if let Err(error) = result {
        return Json(ApiResponse::Error(ApiErrorResponse::new(
            ApiErrorCode::InternalServerError,
            Some(error.to_string()),
        )));
    }

    error!("not implemented at {}:{}", file!(), line!());
    Json(ApiResponse::Error(ApiErrorResponse::new(
        ApiErrorCode::InternalServerError,
        Some("not implemented".to_string()),
    )))
}

pub async fn get_books() -> Json<ApiResponse> {
    let library = get_library();
    if let Err(err) = library {
        return Json(err);
    }

    error!("not implemented at {}:{}", file!(), line!());
    Json(ApiResponse::Error(ApiErrorResponse::new(
        ApiErrorCode::InternalServerError,
        Some("not implemented".to_string()),
    )))
}

pub async fn get_book_by_id(extract::Path(_id): extract::Path<u32>) -> Json<ApiResponse> {
    let library = get_library();
    if let Err(err) = library {
        return Json(err);
    }

    error!("not implemented at {}:{}", file!(), line!());
    Json(ApiResponse::Error(ApiErrorResponse::new(
        ApiErrorCode::InternalServerError,
        Some("not implemented".to_string()),
    )))
}

pub async fn update_book(
    extract::Path(_id): extract::Path<u32>,
    extract::Json(_book): extract::Json<Book>,
) -> Json<ApiResponse> {
    let library = get_library();
    if let Err(err) = library {
        return Json(err);
    }

    error!("not implemented at {}:{}", file!(), line!());
    Json(ApiResponse::Error(ApiErrorResponse::new(
        ApiErrorCode::InternalServerError,
        Some("not implemented".to_string()),
    )))
}

pub async fn drop_book(extract::Path(_id): extract::Path<u32>) -> Json<ApiResponse> {
    let library = get_library();
    if let Err(err) = library {
        return Json(err);
    }

    error!("not implemented at {}:{}", file!(), line!());
    Json(ApiResponse::Error(ApiErrorResponse::new(
        ApiErrorCode::InternalServerError,
        Some("not implemented".to_string()),
    )))
}
