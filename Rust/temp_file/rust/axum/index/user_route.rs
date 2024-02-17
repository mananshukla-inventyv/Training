use axum::{routing::post, Router};

use super::super::service::user_service::{
    add_user, delete_user, get_all_user, get_user, update_user,
};

pub fn user_routes() -> Router {
    let routes = Router::new()
        .route("/add", post(add_user))
        .route("/update", post(update_user))
        .route("/delete", post(delete_user))
        .route("/getall", post(get_all_user))
        .route("/get/:id", post(get_user));

    let user_route = Router::new().nest("/user", routes);

    user_route
}
