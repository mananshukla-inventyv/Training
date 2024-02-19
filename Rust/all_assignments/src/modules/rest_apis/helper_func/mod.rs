use axum::{extract::Path, middleware, response::Response, routing::post, Json, Router};


use self::common_funcs::{get_all, get_item};

pub mod common_funcs;

pub fn common_routes() -> Router {
    let crud_routes = Router::new().route("/get_all", post(get_all))
    .route("/get/:id", post(get_item));


    Router::new()
    .nest("/:route_type", crud_routes)
    .route("/", post(|| async{"Welcome to Home"}))
    // .layer(middleware::from_fn(call))
}




