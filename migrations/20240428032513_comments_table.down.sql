-- Add down migration script here
drop index if exists comments_update_at_index;
drop table if exists comments;
