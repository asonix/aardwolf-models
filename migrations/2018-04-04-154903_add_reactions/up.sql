-- Your SQL goes here
CREATE TABLE reactions (
  id SERIAL PRIMARY KEY,
  reaction_type VARCHAR(10) NOT NULL,
  comment_id INTEGER REFERENCES comments(id) ON DELETE CASCADE NOT NULL
);
