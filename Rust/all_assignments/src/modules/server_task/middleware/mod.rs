use axum::{middleware, Router};

use self::cors::{ cors_layer};

pub mod cors;
pub fn get_middleware()->Router{
    Router::new().route_layer(cors_layer())
}