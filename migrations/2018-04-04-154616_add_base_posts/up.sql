-- Your SQL goes here
CREATE TABLE base_posts (
  id SERIAL PRIMARY KEY,
  name VARCHAR(140),
  media_type VARCHAR(80),
  posted_by INTEGER REFERENCES base_actors(id) ON DELETE CASCADE,
  icon INTEGER REFERENCES images(id) ON DELETE CASCADE,
  original_json JSONB NOT NULL
);
