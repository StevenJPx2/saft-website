-- Your SQL goes here
CREATE TABLE posts (
  id UUID PRIMARY KEY,
  title VARCHAR NOT NULL,
  date DATE NOT NULL,
  body TEXT NOT NULL,
  author VARCHAR NOT NULL,
  published BOOL NOT NULL
);
