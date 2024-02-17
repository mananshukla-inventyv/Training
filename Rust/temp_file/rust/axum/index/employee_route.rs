use axum::{routing::post, Router};
use serde::{Deserialize, Serialize};

use super::super::service::employee_service::{
    add_employee, delete_employee, get_all_employee, get_employee, update_employee
};

pub fn employee_routes() -> Router {
    let routes = Router::new()
        .route("/add", post(add_employee))
        .route("/update", post(update_employee))
        .route("/delete", post(delete_employee))
        .route("/getall", post(get_all_employee))
        .route("/get/:id", post(get_employee));

    let employee_route = Router::new().nest("/employee", routes);

    employee_route
}