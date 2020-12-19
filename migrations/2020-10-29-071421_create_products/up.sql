-- Your SQL goes here
create table products (
    id serial primary key ,
    short_name varchar(128),
    full_name varchar(256),
    description varchar(512),
    vendor_code numeric,
    purchase_price decimal,
    selling_price decimal,
    default_currency integer references currencies(id) on update no action on delete set null,
    default_unit integer references units(id) on update no action on delete set null
)