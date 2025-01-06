use axum::{routing::get, Router};
use log::trace;

macro_rules! register_route {
    ($router: ident, $path: literal, $handler: expr) => {
        trace!("- [{}]->[{}]", stringify!($handler), $path);
        $router = $router.route($path, $handler);
    };
}

pub fn register_routes(mut router: Router) -> Router {
    trace!("Registering routes.");
    register_route!(router, "/", get(root));
    router
}

async fn root() -> &'static str {
    "Hi"
}
