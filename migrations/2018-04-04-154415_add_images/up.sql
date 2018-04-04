-- Your SQL goes here
CREATE TABLE images (
  id SERIAL PRIMARY KEY,
  width INTEGER NOT NULL,
  height INTEGER NOT NULL,
  file_id INTEGER REFERENCES files(id) NOT NULL
);
