use axum::{
    extract::{self, Query, State},
    http::HeaderMap,
    routing::{delete, get, post, put},
    Json, Router,
};
use log::debug;

use crate::{
    library::LibraryErrorStatus,
    model::{
        request::{pagination::Pagination, search::BookSearch},
        response::{
            api::{ApiError, ApiErrorCode, ApiResponse},
            book::BookResponse,
            books::BooksResponse,
        },
    },
    orm::book::Book,
    state::AppState,
};

use super::{auth::login_from_headers, Response};

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
    headers: HeaderMap,
    extract::Json(book): extract::Json<Book>,
) -> Response<String> {
    login_from_headers(&state.db(), &headers).await?;

    let database = state.db();
    state.library_mut().add_book(book, &database).await?;
    Ok(Json(ApiResponse::success(None)))
}

pub async fn get_books(
    State(mut state): State<AppState>,
    pagination: Query<Pagination>,
    search: Query<BookSearch>,
    headers: HeaderMap,
) -> Response<BooksResponse> {
    login_from_headers(&state.db(), &headers).await?;

    let database = state.db();
    Ok(Json(ApiResponse::success(Some(BooksResponse {
        books: state
            .library_mut()
            .get_books(&database, pagination.0, search.0)
            .await?,
    }))))
}

pub async fn get_book_by_id(
    State(mut state): State<AppState>,
    extract::Path(id): extract::Path<i32>,
    headers: HeaderMap,
) -> Response<BookResponse> {
    login_from_headers(&state.db(), &headers).await?;

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
    headers: HeaderMap,
    extract::Json(book): extract::Json<Book>,
) -> Response<String> {
    login_from_headers(&state.db(), &headers).await?;

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
    headers: HeaderMap,
) -> Response<String> {
    login_from_headers(&state.db(), &headers).await?;

    let database = state.db();
    state.library_mut().drop_book(id, &database).await?;
    Ok(Json(ApiResponse::success(None)))
}
