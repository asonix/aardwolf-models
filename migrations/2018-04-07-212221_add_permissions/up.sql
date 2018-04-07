-- Your SQL goes here
CREATE TABLE permissions (
    id SERIAL PRIMARY KEY,
    name VARCHAR(256) UNIQUE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);

INSERT INTO permissions (name, created_at) VALUES ('follow-user', 'now');
INSERT INTO permissions (name, created_at) VALUES ('make-post', 'now');
INSERT INTO permissions (name, created_at) VALUES ('configure-instance', 'now');
INSERT INTO permissions (name, created_at) VALUES ('ban-user', 'now');
INSERT INTO permissions (name, created_at) VALUES ('block-instance', 'now');
