use crate::models::privilege::{Privilege, ModifiedPrivilege};
use diesel::{PgConnection, RunQueryDsl, QueryDsl};

pub fn find(conn: &PgConnection) -> Result<Vec<Privilege>, diesel::result::Error> {
    use crate::schema::privileges::dsl::*;

    privileges.load::<Privilege>(conn)
}

pub fn get(conn: &PgConnection, privilege_id: i32) -> Result<Privilege, diesel::result::Error> {
    use crate::schema::privileges::dsl::*;

    privileges.find(privilege_id).first::<Privilege>(conn)
}

pub fn create(conn: &PgConnection, privilege: ModifiedPrivilege) -> Result<Privilege, diesel::result::Error> {
    use crate::schema::privileges;

    diesel::insert_into(privileges::table).values(&privilege).get_result::<Privilege>(conn)
}

pub fn update(conn: &PgConnection, privilege_id: i32, privilege: ModifiedPrivilege) ->
    Result<Privilege, diesel::result::Error> {
    use crate::schema::privileges::dsl::*;

    diesel::update(privileges.find(privilege_id))
        .set(&privilege)
        .get_result::<Privilege>(conn)
}

pub fn delete(conn: &PgConnection, privilege_id: i32) -> Result<usize, diesel::result::Error> {
    use crate::schema::privileges::dsl::*;

    diesel::delete(privileges.find(privilege_id)).execute(conn)
}