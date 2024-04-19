-- Add down migration script here
drop index if exists topics_update_at_index;
drop table if exists topics;
