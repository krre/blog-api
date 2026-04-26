CREATE TABLE IF NOT EXISTS users (
    id bigserial PRIMARY KEY,
    login text NOT NULL UNIQUE,
    name text NOT NULL,
    password text NOT NULL,
    create_ts timestamptz NOT NULL DEFAULT now(),
    update_ts timestamptz NOT NULL DEFAULT now()
);

INSERT INTO users (login, name, password) VALUES('admin', 'Admin', '');
