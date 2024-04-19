-- Add up migration script here
create table if not exists topics (
    _id uuid NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    comments uuid[] NOT NULL DEFAULT ARRAY[]::uuid[],
    content TEXT NOT NULL,
    create_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    favorite INT NOT NULL DEFAULT 0,
    tags TEXT[] NOT NULL DEFAULT ARRAY[]::TEXT[],
    title TEXT NOT NULL,
    update_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    user_id uuid NOT NULL references users(_id)
);

create unique index if not exists topics_update_at_index on topics(update_at desc);
