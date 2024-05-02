-- Add up migration script here
create table if not exists tags (
    _id uuid not null primary key default gen_random_uuid(),
    create_at timestamptz not null default now(),
    tag text not null,
    topics uuid[] not null default array[]::uuid[]
);

create or replace function update_tags(tags_arr text[], topic_id uuid) returns void as
$$
declare
    tag_txt text;
begin
    foreach tag_txt in array tags_arr
    loop
        update tags t
        set topics =
            case
                when topic_id = any(t.topics::uuid[]) then
                    topics
                else
                    array_append(t.topics, topic_id)
            end
        where t.tag = tag_txt;

        if not found then
            insert into tags (tag, topics)
            values (tag_txt, array[topic_id]);
        end if;
    end loop;
end;
$$ language plpgsql;

create unique index if not exists tags_tag_create_at_index on tags(tag, create_at desc);
