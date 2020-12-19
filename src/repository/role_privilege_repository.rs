use diesel::PgConnection;
use diesel::prelude::*;
use crate::models::role_privilege::RolePrivilege;
use crate::models::privilege::Privilege;

pub fn create(conn: &PgConnection, role_id: i32, privileges: Vec<Privilege>) {
    use crate::schema::roles_privileges;

    for privilege in privileges.iter() {
        let role_privilege = RolePrivilege {
            role_id,
            privilege_id: privilege.id
        };
        diesel::insert_into(roles_privileges::table)
            .values(&role_privilege)
            .execute(conn)
            .expect("failed to insert user_role");
    }
}

pub fn delete(conn: &PgConnection, id: i32) -> QueryResult<usize> {
    use crate::schema::roles_privileges::dsl::*;

    diesel::delete(roles_privileges.filter(role_id.eq(id)))
        .execute(conn)
}