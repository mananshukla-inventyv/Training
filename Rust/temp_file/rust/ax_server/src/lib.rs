pub mod routes;
pub mod  buisness_logic;
pub mod req_checker;
use std::net::SocketAddr;

use routes::create_routes;
// use axum::{routing::get, Router};
use tokio::net::TcpListener;

pub async fn run() {
    let app = create_routes();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    // axum::serve(listener,app).await.unwrap();
    axum::serve(listener, app.await).await.unwrap();
}
