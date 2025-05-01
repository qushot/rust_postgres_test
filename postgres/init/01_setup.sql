CREATE TABLE IF NOT EXISTS todo (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid()
  , title TEXT NOT NULL
  , content TEXT
  , done BOOL NOT NULL DEFAULT FALSE
  , version INT NOT NULL DEFAULT 1
  , created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
  , updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);
COMMENT ON TABLE todo IS 'ToDo';
COMMENT ON COLUMN todo.id IS 'ID';
COMMENT ON COLUMN todo.title IS 'タイトル';
COMMENT ON COLUMN todo.content IS '内容';
COMMENT ON COLUMN todo.done IS '完了フラグ';
COMMENT ON COLUMN todo.version IS 'バージョン';
COMMENT ON COLUMN todo.created_at IS '作成日時';
COMMENT ON COLUMN todo.updated_at IS '更新日時';

INSERT INTO todo (title, content, done) VALUES
  ('初期データ1', '初期データ1の内容', FALSE)
  , ('初期データ2', '初期データ2の内容', TRUE)
  , ('初期データ3', '初期データ3の内容', FALSE)
;
