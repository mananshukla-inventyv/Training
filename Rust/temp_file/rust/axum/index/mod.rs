pub mod employee_route;
pub mod student_route;
pub mod user_route;

use axum::Router;

use self::{
    employee_route::employee_routes, student_route::student_routes, user_route::user_routes,
};
use super::super::axum::health_check::get_status_routes;

pub fn get_routes() -> Router {
    let app = Router::new()
        .merge(student_routes())
        .merge(employee_routes())
        .merge(user_routes())
        .merge(get_status_routes());
    app
}
