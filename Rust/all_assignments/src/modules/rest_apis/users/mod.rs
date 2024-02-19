use axum::{routing::post, Router};

use self::service::{delete_users, update_item_user};

pub mod model;
pub mod service;

pub fn user_routes()->Router{
    Router::new().route("/user/update/:id", post(update_item_user))
    .route("/user/delete/:id", post(delete_users))
}



