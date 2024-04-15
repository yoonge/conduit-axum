-- Your SQL goes here
CREATE TABLE users (
  id SERIAL NOT NULL PRIMARY KEY,
  avatar TEXT NOT NULL DEFAULT '/images/typescript.svg',
  bio TEXT,
  birthday TEXT,
  email TEXT NOT NULL,
  favorite TEXT[],
  gender INTEGER,
  nickname TEXT,
  password TEXT NOT NULL,
  phone TEXT,
  position TEXT,
  username TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
