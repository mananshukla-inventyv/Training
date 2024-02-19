use axum::{routing::get, Router};

pub fn health_check() -> Router {
    Router::new().route(
        "/health/check",
        get(|| async { "Hello World.".to_string() }),
    )
}
