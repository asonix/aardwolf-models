-- Your SQL goes here
CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  content TEXT NOT NULL,
  source TEXT,
  base_post INTEGER REFERENCES base_posts(id) ON DELETE CASCADE NOT NULL
);
