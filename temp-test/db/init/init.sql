-- テーブル作成例
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE
);

-- 初期データ投入例
INSERT INTO users (name, email) VALUES
    ('テストユーザー1', 'test1@example.com'),
    ('テストユーザー2', 'test2@example.com'); 