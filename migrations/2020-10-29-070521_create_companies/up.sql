-- Your SQL goes here
create table companies (
    id serial primary key,
    name varchar(256) not null,
    description varchar(512),
    address varchar(512),
    itn     varchar(64),
    okonkh  varchar(64),
    okpo    varchar(64),
    bic     varchar(64),
    bank_account varchar(64)
)