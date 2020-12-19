-- Your SQL goes here
create table units (
    id serial primary key ,
    name varchar(128) not null,
    short_name varchar(32)
)