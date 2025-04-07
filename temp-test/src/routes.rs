use crate::handlers::hello_handler;
use axum::{Router, routing::get};
use sqlx::PgPool;

pub fn create_routes() -> Router<PgPool> {
    Router::new().route("/", get(hello_handler))
}
