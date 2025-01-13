use std::sync::Arc;

use axum::{
    extract::{self, Query, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use axum_macros::debug_handler;
use log::debug;
use tokio::sync::Mutex;

use crate::{
    library::LibraryErrorStatus,
    model::{
        request::{pagination::Pagination, search::BookSearch},
        response::{
            add_book::AddBookResponse,
            api::{ApiError, ApiErrorCode, ApiResponse},
            book::BookResponse,
            books::GetBooksResponse,
            drop_book::DropBookResponse,
            update_book::UpdateBookResponse,
        },
    },
    orm::book::Book,
    state::AppState,
};

use super::{login::ApiUser, Response};

impl From<LibraryErrorStatus> for Json<ApiResponse<ApiError>> {
    fn from(value: LibraryErrorStatus) -> Self {
        Json(ApiResponse::error(ApiError::new(
            ApiErrorCode::InternalServerError,
            value.to_string(),
        )))
    }
}

pub fn library_router() -> Router<Arc<Mutex<AppState>>> {
    debug!("Registering library router");
    Router::new()
        .route("/", get(get_books))
        .route("/{id}", get(get_book_by_id))
        .route("/{id}", post(add_book))
        .route("/{id}", put(update_book))
        .route("/{id}", delete(drop_book))
}

#[debug_handler]
pub async fn add_book(
    State(state): State<Arc<Mutex<AppState>>>,
    ApiUser(_): ApiUser,
    extract::Json(book): extract::Json<Book>,
) -> Response<AddBookResponse> {
    let mut state = state.lock().await;

    let database = state.db();
    state.library_mut().add_book(book, &database).await?;
    Ok(Json(ApiResponse::success(AddBookResponse)))
}

pub async fn get_books(
    State(state): State<Arc<Mutex<AppState>>>,
    ApiUser(_): ApiUser,
    pagination: Query<Pagination>,
    search: Query<BookSearch>,
) -> Response<GetBooksResponse> {
    let mut state = state.lock().await;

    let database = state.db();
    Ok(Json(ApiResponse::success(GetBooksResponse {
        books: state
            .library_mut()
            .get_books(&database, pagination.0, search.0)
            .await?,
    })))
}

pub async fn get_book_by_id(
    State(state): State<Arc<Mutex<AppState>>>,
    ApiUser(_): ApiUser,
    extract::Path(id): extract::Path<u64>,
) -> Response<BookResponse> {
    let mut state = state.lock().await;

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

    Ok(Json(ApiResponse::success(BookResponse {
        book: book.clone(),
    })))
}

pub async fn update_book(
    State(state): State<Arc<Mutex<AppState>>>,
    ApiUser(_): ApiUser,
    extract::Path(id): extract::Path<u64>,
    extract::Json(book): extract::Json<Book>,
) -> Response<UpdateBookResponse> {
    let mut state = state.lock().await;

    if book.id != id {
        return Err(Json(ApiResponse::error(ApiError::new(
            ApiErrorCode::BadRequest,
            "book id mismatch".to_string(),
        ))));
    }
    let database = state.db();
    state.library_mut().update_book(book, &database).await?;
    Ok(Json(ApiResponse::success(UpdateBookResponse)))
}

pub async fn drop_book(
    State(state): State<Arc<Mutex<AppState>>>,
    ApiUser(_): ApiUser,
    extract::Path(id): extract::Path<u64>,
) -> Response<DropBookResponse> {
    let mut state = state.lock().await;

    let database = state.db();
    state.library_mut().drop_book(id, &database).await?;
    Ok(Json(ApiResponse::success(DropBookResponse)))
}
