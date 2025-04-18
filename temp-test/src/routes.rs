use crate::handlers::hello_handler;
use axum::{Router, routing::get};

// pub fn create_routes() -> Router<PgPool> {
pub fn create_routes() -> Router {
    Router::new().route("/", get(hello_handler))
}
