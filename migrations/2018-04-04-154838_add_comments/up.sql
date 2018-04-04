-- Your SQL goes here
CREATE TABLE comments (
  id SERIAL PRIMARY KEY,
  conversation INTEGER REFERENCES posts(id) NOT NULL,
  parent INTEGER REFERENCES posts(id) NOT NULL,
  post INTEGER REFERENCES posts(id) NOT NULL
);
