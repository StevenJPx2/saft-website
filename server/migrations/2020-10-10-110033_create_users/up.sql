-- Your SQL goes here
CREATE TABLE users (
  id UUID PRIMARY KEY,
  user_name VARCHAR NOT NULL,
  full_name VARCHAR NOT NULL,
  descr TEXT NOT NULL,
  email TEXT NOT NULL,
  password_hash VARCHAR NOT NULL,
  article_ids UUID[] NOT NULL,
  draft_ids UUID[] NOT NULL
);
