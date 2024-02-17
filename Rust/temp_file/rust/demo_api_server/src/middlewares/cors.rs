use axum::http::HeaderName;
use hyper::header;
use tower_http::cors::{AllowOrigin, CorsLayer};

pub fn get_cors_middleware() -> CorsLayer {
    let allowed_headers = vec![
        header::CONTENT_TYPE
    ];
    CorsLayer::new()
        .allow_origin(
            AllowOrigin::any()
        )
        .allow_headers(allowed_headers)
        .allow_methods([
            axum::http::Method::POST,
            axum::http::Method::OPTIONS,
            axum::http::Method::GET,
        ])
        // .allow_credentials(AllowCredentials::yes())
        .expose_headers(vec![HeaderName::from_static("xsrf-token")])
}