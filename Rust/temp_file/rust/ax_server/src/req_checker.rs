use axum::{body::Body, extract::Path, http::{ Request, StatusCode}, middleware::Next, response::{IntoResponse, Response}};

pub async fn req_checker(Path(route_type):Path<String>,req:Request<Body>, next:Next)->Response{
    let vec=vec!["users".to_string(),"products".to_string(),"feedbacks".to_string()];
    if vec.contains(&route_type){
        let res = next.run(req).await;
        res
        
    }else{
        (StatusCode::BAD_REQUEST).into_response()
    }
}