use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use super::{CreateUser, Message, User};
use lazy_static::lazy_static;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

lazy_static! {
    pub static ref ALL_USERS: Arc<RwLock<HashMap<i32, String>>> = Arc::new(RwLock::new(HashMap::new()));
}

pub async fn create_user(Json(payload): Json<CreateUser>) -> Response {
    // Create a random number generator
    let mut rng = StdRng::seed_from_u64(42);

    // Generate a random number between 0 and 100
    let random_number: u32 = rng.gen_range(0..=100);
    // println!("{:?}",)
    // insert your application logic here
    ALL_USERS.write().unwrap().insert(random_number as i32,payload.username.clone());
    return {
        Json(Message {
            status: 2000,
            message_key: String::from("success"),
            data: format!("User add with Username{} and id{}",payload.username,random_number)
        })
    }
    .into_response();
    // this will be converted into a JSON response
    // with a status code of `201 Created`
}

pub async fn get_all_users() -> Response {

    return {
        Json(Message {
            status: 2000,
            message_key: String::from("success"),
            data: ALL_USERS.read().unwrap().clone()
        })
    }
    .into_response();
    // this will be converted into a JSON response
    // with a status code of `201 Created`
}
