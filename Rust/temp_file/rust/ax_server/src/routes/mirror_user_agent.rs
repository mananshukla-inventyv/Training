use axum::{extract::Request, http::{HeaderMap, HeaderValue}};

pub async fn mirror_body_user_agent(req:HeaderMap){
        println!("{:?}",req);

}