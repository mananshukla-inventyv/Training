use axum::{routing::post, Router};


use self::{model::Employee, service::{delete_emp, update_item_emp}};

pub mod model;
pub mod service;

pub fn emp_routes()->Router{
    Router::new().route("/employee/update/:id", post(update_item_emp))
    .route("/employee/delete/:id", post(delete_emp))
}