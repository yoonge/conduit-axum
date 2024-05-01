-- Add down migration script here
drop index if exists comments_create_at_index;
drop table if exists comments;
