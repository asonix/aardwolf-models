-- Your SQL goes here
CREATE TABLE follow_requests (
    id SERIAL PRIMARY KEY,
    follower INTEGER REFERENCES base_actors(id) ON DELETE CASCADE NOT NULL,
    requested_follow INTEGER REFERENCES base_actors(id) ON DELETE CASCADE NOT NULL
);
