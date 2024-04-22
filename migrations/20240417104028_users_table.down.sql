-- Add down migration script here
drop trigger if exists users_update_at_trigger on users;
drop index if exists users_username_create_at_index;
drop table if exists users;
