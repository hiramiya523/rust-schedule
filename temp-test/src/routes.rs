use crate::handlers::hello_handler;
use axum::{Router, routing::get};
use sea_orm::DatabaseConnection;

pub fn create_routes() -> Router<DatabaseConnection> {
    Router::new().route("/", get(hello_handler))
}
