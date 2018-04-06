-- Your SQL goes here
CREATE TABLE emails (
    id SERIAL PRIMARY KEY,
    email VARCHAR(256),
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE NOT NULL
);
