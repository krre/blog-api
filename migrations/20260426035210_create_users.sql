CREATE TABLE IF NOT EXISTS users (
    id bigserial PRIMARY KEY,
    username text NOT NULL UNIQUE,
    first_name text NOT NULL,
    last_name text NOT NULL DEFAULT '',
    password_hash text NOT NULL DEFAULT '',
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

INSERT INTO users (username, first_name) VALUES('admin', 'Admin');
