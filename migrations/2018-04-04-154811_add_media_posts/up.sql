-- Your SQL goes here
CREATE TABLE media_posts (
  id SERIAL PRIMARY KEY,
  file_id INTEGER REFERENCES files(id) NOT NULL,
  post_id INTEGER REFERENCES posts(id) NOT NULL
);
