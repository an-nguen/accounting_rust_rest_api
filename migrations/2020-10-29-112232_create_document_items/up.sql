-- Your SQL goes here
create table document_items (
    id serial primary key ,
    product_id integer references products(id) on update no action on delete set null,
    document_id integer references documents(id) on update cascade on delete set null,
    series_code numeric,
    quantity numeric not null default 0,
    price decimal not null default 0,
    total decimal not null default 0,
    currency_id integer references currencies(id) on update no action on delete set null,
    unit_id integer references units(id) on update no action on delete set null
)