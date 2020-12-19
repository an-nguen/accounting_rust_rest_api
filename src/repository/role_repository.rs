use diesel::{PgConnection, RunQueryDsl, QueryDsl};
use crate::models::role::{Role, ModifiedRole, ReqModifiedRole};
use crate::repository::role_privilege_repository;

pub fn find(conn: &PgConnection) -> Result<Vec<Role>, diesel::result::Error> {
    use crate::schema::roles::dsl::*;

    let role_vec = roles.load::<Role>(conn)?;

    Ok(role_vec)
}

pub fn get(conn: &PgConnection, role_id: i32) -> Result<Role, diesel::result::Error> {
    use crate::schema::roles::dsl::*;

    roles.find(role_id).first::<Role>(conn)
}

pub fn create(conn: &PgConnection, req_role: ReqModifiedRole) -> Result<Role, diesel::result::Error> {
    use crate::schema::roles;

    let new_role = ModifiedRole {
        name: req_role.name
    };
    let role = diesel::insert_into(roles::table)
        .values(&new_role)
        .get_result::<Role>(conn)?;
    role_privilege_repository::create(conn, role.id, req_role.privileges);
    Ok(role)
}

pub fn update(conn: &PgConnection, role_id: i32, req_upd_role: ReqModifiedRole) -> Result<Role, diesel::result::Error> {
    use crate::schema::roles::dsl::*;

    let upd_role = ModifiedRole {
        name: req_upd_role.name
    };
    let role = diesel::update(roles.find(role_id))
        .set(&upd_role)
        .get_result::<Role>(conn)?;
    role_privilege_repository::delete(conn, role.id).expect("failed to delete role privilege");
    role_privilege_repository::create(conn, role_id, req_upd_role.privileges);
    Ok(role)
}

pub fn delete(conn: &PgConnection, role_id: i32) -> Result<usize, diesel::result::Error> {
    use crate::schema::roles::dsl::*;

    role_privilege_repository::delete(conn, role_id).expect("failed to role_privilege");
    diesel::delete(roles.find(role_id))
        .execute(conn)
}