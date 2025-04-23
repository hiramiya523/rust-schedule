use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
}

// サーバーの設定
#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
}

// #[derive(Debug, Deserialize)]
// pub struct DatabaseConfig {
//     pub enabled: bool,
//     pub url: String,
// }

// #[derive(Debug, Deserialize)]
// pub struct LoggingConfig {
//     pub level: String,
// }

impl Config {
    // tomlファイルを読み込む
    /**
     * 設定ファイルを読み込む
     */
    pub fn load() -> Self {
        // let config_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config/default.toml".to_string());
        let config_path = "config/default.toml";
        // 文字列として読み込む
        let config_str = fs::read_to_string(config_path).expect("Failed to read config file");
        // TがDeserializeOwnedを実装していることを要求する
        // ResultのTの型が決定するタイミングは?
        // これは、型アノテーションか、型推論かによって決定する。
        // 今回はload()メソッドの戻り値がConfig型なので、型推論される。
        // 型推論されるということは、型アノテーションが不要になるということ。
        toml::from_str(&config_str).expect("Failed to parse config file")
    }
}
