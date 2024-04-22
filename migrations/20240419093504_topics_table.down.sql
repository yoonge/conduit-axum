-- Add down migration script here
drop trigger if exists topics_update_at_trigger on topics;
drop index if exists topics_update_at_index;
drop table if exists topics;
