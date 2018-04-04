-- Your SQL goes here
CREATE TABLE base_posts (
  id SERIAL PRIMARY KEY,
  name VARCHAR(140),
  media_type VARCHAR(80),
  posted_by INTEGER REFERENCES base_actors(id),
  icon INTEGER REFERENCES images(id),
  original_json JSONB NOT NULL
);
