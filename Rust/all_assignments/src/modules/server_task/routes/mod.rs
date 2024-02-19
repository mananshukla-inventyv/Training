use axum::{middleware, Router};

use super::{employee::emp_routes, health_check::health_check, helper_func::common_routes, middleware::{cors::{ cors_layer, type_middelware}, get_middleware}, student::student_routes, users::user_routes};


pub async fn create_routes() -> Router {
    Router::new().merge(health_check()).merge(common_routes()).merge(student_routes()).merge(user_routes()).merge(emp_routes()).layer(cors_layer())
}
