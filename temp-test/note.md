## rust

### sea-orm

#### マイグレーション

sea-orm-cliをインストール

```bash
cargo install sea-orm-cli
```

マイグレーションの初期化

```bash
sea-orm-cli migrate init
```

マイグレーションの生成

```bash
sea-orm-cli migrate generate create_posts_table
```

マイグレーションの実行

```bash
sea-orm-cli migrate run
```

sea-ormのインストール

```bash
cargo add sea-orm --features sqlx-postgres tokio macros
```

## postgres

### 接続

```bash
psql -h localhost -U postgres -d rust_schedule
psql -U postgres -d rust_schedule
```

データベースの一覧

```bash
\l
```

テーブルの一覧

```bash
\d
```

テーブルの詳細

```bash
\d テーブル名
```

### テーブルの作成
```bash
CREATE TABLE posts (
```