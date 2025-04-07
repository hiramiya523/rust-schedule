mod db;
mod handlers;
mod models;
mod routes;

use dotenv::dotenv;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Load .env file
    dotenv().ok();

    // Initialize logging
    tracing_subscriber::fmt::init();

    // データベース接続の作成
    let db = db::establish_connection().await;

    // build our application with a single route

    let app = routes::create_routes().with_state(db.clone());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

// async fn plain_text() -> &'static str { "foo"
// }
