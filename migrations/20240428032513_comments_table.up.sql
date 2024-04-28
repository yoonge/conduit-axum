-- Add up migration script here
create table if not exists comments (
    _id uuid not null primary key default gen_random_uuid(),
    content text not null,
    create_at timestamptz not null default now(),
    topic uuid not null references topics(_id),
    user_id uuid not null references users(_id)
);

create unique index if not exists comments_create_at_index on comments(create_at desc);
