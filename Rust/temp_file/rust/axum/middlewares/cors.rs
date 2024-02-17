use axum::http::HeaderName;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};

pub fn get_cors_middleware() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(AllowOrigin::any())
        .allow_headers(Any)
        .allow_methods([
            axum::http::Method::POST,
            axum::http::Method::OPTIONS,
            axum::http::Method::GET,
        ])
        // .allow_credentials(AllowCredentials::yes())
        .expose_headers(vec![HeaderName::from_static("xsrf-token")])
}
