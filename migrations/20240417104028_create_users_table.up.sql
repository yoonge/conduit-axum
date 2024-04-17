-- Add up migration script here
create table users (
  id SERIAL NOT NULL PRIMARY KEY,
  avatar TEXT NOT NULL DEFAULT '/images/typescript.svg',
  bio TEXT NOT NULL DEFAULT '',
  birthday TEXT NOT NULL DEFAULT '',
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  email TEXT NOT NULL,
  favorite INTEGER[] NOT NULL DEFAULT ARRAY[]::INTEGER[],
  -- 1: male, 0: female, -1: secret
  gender SMALLINT NOT NULL DEFAULT -1,
  nickname TEXT NOT NULL DEFAULT '',
  password TEXT NOT NULL,
  phone TEXT NOT NULL DEFAULT '',
  position TEXT NOT NULL DEFAULT '',
  username TEXT NOT NULL
);

create unique index users_email_index on users(email);
