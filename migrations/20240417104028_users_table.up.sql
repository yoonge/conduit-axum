-- Add up migration script here
create table if not exists users (
    _id uuid NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    avatar TEXT NOT NULL DEFAULT '/images/typescript.svg',
    bio TEXT NOT NULL DEFAULT '',
    birthday TEXT NOT NULL DEFAULT '',
    create_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    email TEXT NOT NULL,
    favorite uuid[] NOT NULL DEFAULT ARRAY[]::uuid[],
    -- 1: male, 0: female, -1: secret
    gender SMALLINT NOT NULL DEFAULT -1,
    nickname TEXT NOT NULL DEFAULT '',
    password TEXT NOT NULL,
    phone TEXT NOT NULL DEFAULT '',
    position TEXT NOT NULL DEFAULT '',
    update_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    username TEXT NOT NULL
);

create or replace function update_at_column() returns trigger as $$
begin
    new.update_at = now();
    return new;
end;
$$ language plpgsql;

create trigger users_update_at_trigger
before update on users
for each row execute procedure update_at_column();

create unique index if not exists users_username_create_at_index on users(username, create_at desc);
