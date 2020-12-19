-- Your SQL goes here
create table users_roles (
    user_id integer references users(id) on update cascade on delete cascade ,
    role_id integer references roles(id) on update cascade on delete cascade,
    primary key (user_id, role_id)
)