-- Your SQL goes here
CREATE TABLE links (
  id SERIAL PRIMARY KEY,
  href VARCHAR(2048) NOT NULL,
  href_lang VARCHAR(8) NOT NULL,
  height INTEGER,
  width INTEGER,
  preview TEXT,
  base_post INTEGER REFERENCES base_posts(id) NOT NULL
);
