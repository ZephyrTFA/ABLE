use axum::{
    response::Redirect,
    routing::{delete, get, post, put},
    Router,
};
use library::{add_book, drop_book, get_book_by_id, get_books, update_book};
use log::trace;

use crate::state::AppState;

pub mod library;

macro_rules! register_route {
    ($router: ident, $path: literal, $handler: expr) => {
        trace!("- [{}]->[{}]", stringify!($handler), $path);
        $router = $router.route($path, $handler);
    };
}

pub fn init_router(state: AppState) -> Router {
    trace!("Registering routes.");
    let mut router = Router::new();

    register_route!(router, "/", get(root));
    register_route!(router, "/books", get(get_books));
    register_route!(router, "/books/{id}", get(get_book_by_id));
    register_route!(router, "/books/{id}", post(add_book));
    register_route!(router, "/books/{id}", put(update_book));
    register_route!(router, "/books/{id}", delete(drop_book));
    router.with_state(state)
}

async fn root() -> Redirect {
    Redirect::permanent("/viewer")
    // todo: implement front-end viewer
}
