CREATE TABLE IF NOT EXISTS posts (
    id bigserial PRIMARY KEY,
    user_id int8 NOT NULL REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE,
    title text NOT NULL,
    post text NOT NULL DEFAULT '',
    is_published bool NOT NULL DEFAULT false,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    published_at timestamptz
);

CREATE INDEX IF NOT EXISTS posts_user_id_idx ON posts (user_id);
