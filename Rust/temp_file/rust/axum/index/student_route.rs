use axum::{routing::post, Router};

use super::super::service::student_service::{
    add_student, delete_student, get_all_student, get_student, update_student,
};

pub fn student_routes() -> Router {
    let routes = Router::new()
        .route("/add", post(add_student))
        .route("/update", post(update_student))
        .route("/delete", post(delete_student))
        .route("/getall", post(get_all_student))
        .route("/get/:id", post(get_student));

    let student_route = Router::new().nest("/student", routes);

    student_route
}
