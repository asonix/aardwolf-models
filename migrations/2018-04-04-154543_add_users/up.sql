-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  password VARCHAR(256) NOT NULL,
  created_at TIMESTAMPTZ NOT NULL
);
