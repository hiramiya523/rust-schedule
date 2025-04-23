mod config;
mod handlers;
mod routes;
use std::fs::{OpenOptions, create_dir_all};
use std::net::{IpAddr, SocketAddr};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // logsディレクトリを作成
    create_dir_all("logs").expect("Failed to create logs directory");

    // ファイル出力の設定
    let file_appender = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logs/app.log")
        .expect("Failed to open log file");

    // ロギングの初期化（コンソールとファイルの両方に出力）
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "temp_test=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stdout)) // コンソール出力
        .with(tracing_subscriber::fmt::layer().with_writer(file_appender)) // ファイル出力
        .init();

    // 設定の読み込み
    let config = config::Config::load();
    tracing::info!("設定を読み込みました: {:?}", config);

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
    tracing::info!("サーバーを起動します: {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
