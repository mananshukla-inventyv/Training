use axum::{routing::post, Router};
use serde::{Deserialize, Serialize};

use self::user_model::{create_user, get_all_users};
mod user_model;


#[derive(Serialize)]
pub struct Message<T> {
    pub status: u32,
    pub message_key: String,
    pub data: T,
}


pub fn get_user_routes() -> Router { 
    let routees = Router::new().route("/users/add", post(create_user)).route("/get_all", post(get_all_users));
    routees
    
}



// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    pub id: u64,
    pub username: String,
}
