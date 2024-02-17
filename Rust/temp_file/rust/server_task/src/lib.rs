// use routes::create_routes;
use tokio::net::TcpListener;

pub mod config;
pub mod health_check;
pub mod helper_func;
pub mod middleware;
pub mod routes;
pub mod users;

// pub async fn run() {
//     let app = create_routes().await;
//     // let addr=SocketAddr::new([((0,0,0,0),3000)]);
//     let listener = TcpListener::bind(&"0.0.0.0:5000").await.unwrap();
//     axum::serve(listener, app).await.unwrap();
// }
