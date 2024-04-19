-- Add down migration script here
drop index if exists users_username_create_at_index;
drop table if exists users;
