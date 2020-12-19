-- Your SQL goes here
create table currencies (
    id serial primary key ,
    name varchar(32) not null,
    short_name varchar(16)
)