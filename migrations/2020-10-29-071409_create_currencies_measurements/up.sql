-- Your SQL goes here
create table currencies_measurements (
    id serial primary key ,
    currency_id integer references currencies(id) on update no action on delete set null ,
    value decimal not null ,
    date date
)