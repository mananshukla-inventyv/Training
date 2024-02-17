use std::net::SocketAddr;
use middlewares::get_middlewares;

use crate::routes::get_routes;
mod config;
mod user;
mod health_check;
mod routes;
mod middlewares;
#[tokio::main]
async fn main() {
    //let a  = ;
    let app = get_routes();
    let app = get_middlewares(app);
    // let app =app.layer(ServiceBuilder::new().layer(get_cors_middleware()));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let _server = axum::Server::bind(&addr).serve(app.clone().into_make_service()).await.expect("something went wrong!");
}