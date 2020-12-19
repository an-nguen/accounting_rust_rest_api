-- Your SQL goes here
create table roles_privileges (
    role_id integer references roles(id) on update cascade on delete cascade,
    privilege_id integer references privileges(id) on update cascade on delete cascade,
    allow boolean,
    primary key (role_id, privilege_id)
)