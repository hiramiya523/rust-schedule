mod config;
mod handlers;
mod routes;
use std::net::{IpAddr, SocketAddr};

#[tokio::main]
async fn main() {
    // 設定の読み込み
    let config = config::Config::load();

    // ロギングの初期化
    // tracing_subscriber::fmt::init();

    // アプリケーションの構築
    let app = routes::create_routes();
    // let app = if let Some(db) = db {
    //     routes::create_routes().with_state(db)
    // } else {
    //     routes::create_routes()
    // };

    // サーバーの起動
    let addr = SocketAddr::new(
        config.server.host.parse::<IpAddr>().unwrap(),
        config.server.port,
    );
    println!("listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
