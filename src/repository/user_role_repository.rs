use diesel::prelude::*;

use crate::models::role::Role;
use crate::models::user_role::UserRole;

pub fn create(conn: &PgConnection, user_id: i32, roles: Vec<Role>) {
    use crate::schema::users_roles;

    for role in roles.iter() {
        let user_role = UserRole {
            user_id,
            role_id: role.id
        };
        diesel::insert_into(users_roles::table)
            .values(&user_role)
            .execute(conn)
            .expect("failed to insert user_role");
    }
}

pub fn delete(conn: &PgConnection, usr_id: i32) -> QueryResult<usize> {
    use crate::schema::users_roles::dsl::*;

    diesel::delete(users_roles.filter(user_id.eq(usr_id)))
        .execute(conn)
}