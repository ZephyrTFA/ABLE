use axum::{
    extract::{self, State},
    Json,
};
use axum_macros::debug_handler;

use crate::{
    library::{Library, LibraryErrorStatus},
    model::response::{
        api::{ApiError, ApiErrorCode, ApiResponse},
        book::BookResponse,
        books::BooksResponse,
    },
    orm::book::Book,
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

pub async fn add_book(
    State(mut library): State<Library>,
    extract::Json(book): extract::Json<Book>,
) -> Response<String> {
    library.add_book(book).await?;
    Ok(Json(ApiResponse::success(None)))
}

pub async fn get_books(State(library): State<Library>) -> Response<BooksResponse> {
    // this sucks, surely there has to be a better way than cloning the entire list?
    Ok(Json(ApiResponse::success(Some(BooksResponse {
        books: library.get_books().await,
    }))))
}

#[debug_handler]
pub async fn get_book_by_id(
    State(library): State<Library>,
    extract::Path(id): extract::Path<i32>,
) -> Response<BookResponse> {
    let book = library.get_book_by_id(id).await;
    if book.is_err() {
        return match book.unwrap_err() {
            LibraryErrorStatus::DatabaseError => Err(Json(ApiResponse::error(ApiError::new(
                ApiErrorCode::InternalServerError,
                String::new(),
            )))),
            status => Err(Json(ApiResponse::error(ApiError::new(
                ApiErrorCode::NotFound,
                format!("library error: {status}",),
            )))),
        };
    }
    let book = book.unwrap();

    Ok(Json(ApiResponse::success(Some(BookResponse {
        book: book.clone(),
    }))))
}

pub async fn update_book(
    State(mut library): State<Library>,
    extract::Path(id): extract::Path<i32>,
    extract::Json(book): extract::Json<Book>,
) -> Response<String> {
    if book.id != id {
        return Err(Json(ApiResponse::error(ApiError::new(
            ApiErrorCode::BadRequest,
            "book id mismatch".to_string(),
        ))));
    }
    library.update_book(book).await?;
    Ok(Json(ApiResponse::success(None)))
}

pub async fn drop_book(
    State(mut library): State<Library>,
    extract::Path(id): extract::Path<i32>,
) -> Response<String> {
    library.drop_book(id).await?;
    Ok(Json(ApiResponse::success(None)))
}
