-- Your SQL goes here
CREATE TABLE timers (
    id SERIAL PRIMARY KEY,
    fire_time TIMESTAMPTZ NOT NULL
);
