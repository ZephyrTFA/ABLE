use axum::{
    extract::{self, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use axum_macros::debug_handler;
use log::debug;

use crate::{
    library::LibraryErrorStatus,
    model::response::{
        api::{ApiError, ApiErrorCode, ApiResponse},
        book::BookResponse,
        books::BooksResponse,
    },
    orm::book::Book,
    state::AppState,
};

use super::Response;

impl From<LibraryErrorStatus> for Json<ApiResponse<ApiError>> {
    fn from(value: LibraryErrorStatus) -> Self {
        Json(ApiResponse::error(ApiError::new(
            ApiErrorCode::InternalServerError,
            value.to_string(),
        )))
    }
}

pub fn library_router() -> Router<AppState> {
    debug!("Registering library router");
    Router::new()
        .route("/", get(get_books))
        .route("/{id}", get(get_book_by_id))
        .route("/{id}", post(add_book))
        .route("/{id}", put(update_book))
        .route("/{id}", delete(drop_book))
}

pub async fn add_book(
    State(mut state): State<AppState>,
    extract::Json(book): extract::Json<Book>,
) -> Response<String> {
    let database = state.db();
    state.library_mut().add_book(book, &database).await?;
    Ok(Json(ApiResponse::success(None)))
}

pub async fn get_books(State(mut state): State<AppState>) -> Response<BooksResponse> {
    let database = state.db();
    Ok(Json(ApiResponse::success(Some(BooksResponse {
        books: state.library_mut().get_books(&database).await?,
    }))))
}

#[debug_handler]
pub async fn get_book_by_id(
    State(mut state): State<AppState>,
    extract::Path(id): extract::Path<i32>,
) -> Response<BookResponse> {
    let database = state.db();
    let book = state.library_mut().get_book_by_id(id, &database).await;
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
    State(mut state): State<AppState>,
    extract::Path(id): extract::Path<i32>,
    extract::Json(book): extract::Json<Book>,
) -> Response<String> {
    if book.id != id {
        return Err(Json(ApiResponse::error(ApiError::new(
            ApiErrorCode::BadRequest,
            "book id mismatch".to_string(),
        ))));
    }
    let database = state.db();
    state.library_mut().update_book(book, &database).await?;
    Ok(Json(ApiResponse::success(None)))
}

pub async fn drop_book(
    State(mut state): State<AppState>,
    extract::Path(id): extract::Path<i32>,
) -> Response<String> {
    let database = state.db();
    state.library_mut().drop_book(id, &database).await?;
    Ok(Json(ApiResponse::success(None)))
}
