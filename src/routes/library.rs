use std::sync::{Mutex, MutexGuard};

use axum::{extract, Json};
use log::{trace, warn};
use once_cell::sync::Lazy;

use crate::{
    library::{Library, LibraryErrorStatus},
    model::response::{
        api::{ApiError, ApiErrorCode, ApiResponse},
        book::{Book, BookResponse, BooksResponse},
    },
};

type Response<T> = Result<Json<ApiResponse<T>>, Json<ApiResponse<ApiError>>>;

impl From<LibraryErrorStatus> for Json<ApiResponse<ApiError>> {
    fn from(value: LibraryErrorStatus) -> Self {
        Json(ApiResponse::error(ApiError::new(
            ApiErrorCode::InternalServerError,
            value.to_string(),
        )))
    }
}

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

pub async fn add_book(extract::Json(book): extract::Json<Book>) -> Response<String> {
    let mut library = get_library()?;
    library.add_book(book)?;
    Ok(Json(ApiResponse::success(None)))
}

pub async fn get_books() -> Response<BooksResponse> {
    let library = get_library()?;
    // this sucks, surely there has to be a better way than cloning the entire list?
    let books = library.get_books().cloned().collect::<Vec<_>>();
    let response = BooksResponse::from(books);

    Ok(Json(ApiResponse::success(Some(response))))
}

pub async fn get_book_by_id(extract::Path(id): extract::Path<i32>) -> Response<BookResponse> {
    let library = get_library()?;
    let book = library.get_book_by_id(&id);
    if book.is_none() {
        return Err(Json(ApiResponse::error(ApiError::new(
            ApiErrorCode::NotFound,
            "book id not found".to_string(),
        ))));
    }
    let book = book.unwrap();

    Ok(Json(ApiResponse::success(Some(BookResponse::from(book)))))
}

pub async fn update_book(
    extract::Path(id): extract::Path<i32>,
    extract::Json(book): extract::Json<Book>,
) -> Response<String> {
    let mut library = get_library()?;
    if book.id() != &id {
        return Err(Json(ApiResponse::error(ApiError::new(
            ApiErrorCode::BadRequest,
            "book id mismatch".to_string(),
        ))));
    }
    library.update_book(book)?;
    Ok(Json(ApiResponse::success(None)))
}

pub async fn drop_book(extract::Path(id): extract::Path<i32>) -> Response<String> {
    let mut library = get_library()?;
    library.drop_book(&id)?;
    Ok(Json(ApiResponse::success(None)))
}
