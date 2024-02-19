use std::vec;
use tower_http::cors::{Any, CorsLayer};
use axum::{body::{Body, HttpBody}, extract::{Path, Request}, http::{HeaderMap, HeaderName, Method, StatusCode}, middleware::Next, response::{IntoResponse, Response}};
pub fn cors_layer() -> CorsLayer {
        // let map = HeaderMap::new();

    CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods([Method::GET, Method::POST])
        .expose_headers(vec![HeaderName::from_static("xsrf")])
}


    pub async fn type_middelware(Path(var):Path<(String,i32)>,req:Request<Body>, next:Next)->Result<Response<Body>,StatusCode>{
        let vec=vec!["user".to_string(),"student".to_string(),"employee".to_string()];
        if vec.contains(&var.0){
            let res=next.run(req).await;
            Ok(res)
        }
        else {
            Err(StatusCode::BAD_REQUEST)
        }
    }
    // axum::middleware::from_fn(f)
