use axum::Router;
use crate::health_check::get_status_routes;
use crate::user::get_user_routes;




pub fn get_routes() -> Router { 
    let app = Router::new().merge(get_status_routes()).merge(get_user_routes());
    app
}