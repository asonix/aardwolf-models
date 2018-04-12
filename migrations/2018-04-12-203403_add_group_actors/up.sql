-- Your SQL goes here
CREATE TABLE group_actors (
  id SERIAL PRIMARY KEY,
  group_id INTEGER REFERENCES groups(id) ON DELETE CASCADE NOT NULL,
  base_actor_id INTEGER REFERENCES base_actors(id) ON DELETE CASCADE NOT NULL
);
