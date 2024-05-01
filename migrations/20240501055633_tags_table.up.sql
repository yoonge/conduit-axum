-- Add up migration script here
create table if not exists tags (
    _id uuid not null primary key default gen_random_uuid(),
    create_at timestamptz not null default now(),
    tag text not null,
    topics uuid[] not null default array[]::uuid[]
);

create unique index if not exists tags_tag_create_at_index on tags(tag, create_at desc);
