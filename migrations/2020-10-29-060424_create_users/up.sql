-- Your SQL goes here
create table users
(
    id serial primary key,
    username varchar(64) not null,
    password varchar(128) not null,
    email varchar(64) default '',
    first_name varchar(64) default '',
    last_name varchar(64) default '',
    birthday date,
    valid_to date
);
insert into users (username, password, birthday, valid_to) values ('admin', '', null, null);