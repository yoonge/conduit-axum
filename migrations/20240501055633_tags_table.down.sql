-- Add down migration script here
drop index if exists tags_tag_create_at_index;
drop table if exists tags;
