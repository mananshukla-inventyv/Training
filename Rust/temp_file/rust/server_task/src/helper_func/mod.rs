// use axum::{routing::post, Router};
// use tower::{Layer, ServiceBuilder};

// use crate::{middleware::cors::{cors_layer, type_middelware}, users::user_models::{create_item, delete_item, get_all, get_item, update_item_emp, update_item_student, update_item_user, Employee, Student}};

// pub fn crud_ops() -> Router {
//     // let crud_routes = Router::new()
//     //     .route("/:id", post(get_item))
        
//     //     .route("/delete/:id", post(delete_item));
//         // .route("/add", post(update_item_emp));
//     // let get_all_route=Router::new().route("/get_all", post(get_all));
//     // Router::new()
//     // // .nest("/:route_type", get_all_route)
//     // .route("/student/update/:id", post(update_item_student))
//     // .route("/employee/update/:id", post(update_item_emp))
//     // .route("/user/update/:id", post(update_item_user))
//     // // .nest("/:route_type", crud_routes)
//     //     .route("/", post(|| async { "Welcome to home" }))
//     //     .layer(cors_layer())
//     //     .layer(axum::middleware::from_fn(type_middelware))

//     let routes = Router::new()
//     .route("/add", post(add_student))
//     .route("/update", post(update_student))
//     .route("/delete", post(delete_student))
//     .route("/getall", post(get_all_student))
//     .route("/get/:id", post(get_student));
    
// }

