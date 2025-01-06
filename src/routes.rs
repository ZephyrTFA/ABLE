use axum::{routing::get, Router};

macro_rules! register_route {
    ($router: ident, $path: literal, $handler: expr) => {
        println!("- [{}]->[{}]", stringify!($handler), $path);
        $router = $router.route($path, $handler);
    };
}

pub fn register_routes(mut router: Router) -> Router {
    println!("Registering routes.");
    register_route!(router, "/", get(root));
    router
}

async fn root() -> &'static str {
    "Hi"
}
