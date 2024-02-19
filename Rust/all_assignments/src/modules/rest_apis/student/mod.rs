use axum::{routing::post, Router};

use super::employee::service::{delete_emp, update_item_emp};


pub mod model;
pub mod service;

pub fn student_routes()->Router{
    Router::new().route("/student/update/:id", post(update_item_emp))
    .route("/student/delete/:id", post(delete_emp))
}