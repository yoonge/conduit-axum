-- Add down migration script here
drop function if exists update_tags(text[], uuid);
drop index if exists tags_tag_create_at_index;
drop table if exists tags;
