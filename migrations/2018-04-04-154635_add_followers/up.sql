-- Your SQL goes here
CREATE TABLE followers (
  id SERIAL PRIMARY KEY,
  follower INTEGER REFERENCES base_actors(id) ON DELETE CASCADE NOT NULL,
  follows INTEGER REFERENCES base_actors(id) ON DELETE CASCADE NOT NULL
);
