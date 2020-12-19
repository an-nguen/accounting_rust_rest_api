use diesel::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

use crate::models::company::{Company, ModifiedCompany};

pub fn find(conn: &PgConnection) -> Result<Vec<Company>, Error> {
    use crate::schema::companies;

    find_all!(companies::table, conn)
}

pub fn get(conn: &PgConnection, id: i32)
           -> Result<Company, Error> {
    use crate::schema::companies;

    get_one!(companies::table, conn, id)
}

pub fn create(conn: &PgConnection, company: ModifiedCompany)
              -> Result<Company, Error> {
    use crate::schema::companies;

    create!(companies::table, conn, &company)
}

pub fn update(conn: &PgConnection, id: i32, company: ModifiedCompany)
              -> Result<Company, Error> {
    use crate::schema::companies;

    update!(companies::table, conn, id, &company)
}

pub fn delete(conn: &PgConnection, id: i32) -> Result<usize, Error> {
    use crate::schema::companies;

    delete!(companies::table, conn, id)
}