-- Your SQL goes here
CREATE TABLE direct_posts (
  id SERIAL PRIMARY KEY,
  base_post_id INTEGER REFERENCES base_posts(id) ON DELETE CASCADE NOT NULL,
  base_actor_id INTEGER REFERENCES base_actors(id) ON DELETE CASCADE NOT NULL
);
