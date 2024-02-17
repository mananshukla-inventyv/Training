use axum::{
    routing::{delete, get, post},
    Router,
};
use axum_server::service::SendService;
use tower::ServiceBuilder;

use crate::buisness_logic::{self, create_item, delete_item, get_all, get_item, update_item};
use crate::req_checker::req_checker;
use hello_world::return_string;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use path_variable::path_variable;
use query_parameters::query_parameters;

use self::mirror_user_agent::mirror_body_user_agent;
mod hello_world;
mod mirror_body_json;
mod mirror_body_string;
mod path_variable;
mod query_parameters;
mod mirror_user_agent;

pub async fn create_routes() -> Router {
    Router::new()
    .route("/", get(return_string))
    .route("/", post(return_string))
    // .route("/", post(return_string))
    .route("/mirror_body_string", post(mirror_body_string))
    .route("/mirror_body_json", post(mirror_body_json))
    .route("/path_var/:id/:name", get(path_variable))
    .route("/query_params", post(query_parameters))
    .route("/user_agent", post(mirror_body_user_agent))
}

//     let crud_routes = Router::new()
//         .route(
//             "/:id",
//             get(get_item)
//                 .patch(update_item)
//                 .delete(delete_item),
//         )
//         .route(
//             "/",
//             get(get_all)
//             .post(create_item),
//         );
   

//     Router::new()
//     .nest("/:route_type", crud_routes)
//     .route("/", get(|| async { "Welcome to home" }))
    
// }
