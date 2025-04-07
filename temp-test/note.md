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

## postgres

### 接続

```bash
psql -h localhost -U postgres -d mydatabase
psql -U postgres -d mydatabase
```

### テーブルの作成

```bash
CREATE TABLE posts (
```