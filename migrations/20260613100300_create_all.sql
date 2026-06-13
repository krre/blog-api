CREATE TABLE IF NOT EXISTS users (
    id bigserial PRIMARY KEY,
    username text NOT NULL UNIQUE,
    first_name text NOT NULL,
    last_name text NOT NULL DEFAULT '',
    password_hash text NOT NULL DEFAULT '',
    email text NOT NULL UNIQUE DEFAULT '',
    location text NOT NULL DEFAULT '',
    bio text NOT NULL DEFAULT '',
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

INSERT INTO users (username, first_name) VALUES('admin', 'Admin');

CREATE TABLE IF NOT EXISTS posts (
    id bigserial PRIMARY KEY,
    user_id int8 NOT NULL REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE,
    title text NOT NULL,
    post text NOT NULL DEFAULT '',
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    published_at timestamptz
);

CREATE INDEX IF NOT EXISTS posts_user_id_idx ON posts (user_id);
