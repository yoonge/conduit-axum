-- Add down migration script here
drop index if exists users_email_index;
drop table if exists users;
