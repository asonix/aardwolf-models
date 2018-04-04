-- Your SQL goes here
CREATE TABLE followers (
  id SERIAL PRIMARY KEY,
  follower INTEGER REFERENCES base_actors(id) NOT NULL,
  follows INTEGER REFERENCES base_actors(id) NOT NULL
);
