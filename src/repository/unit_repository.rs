use diesel::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

use crate::models::unit::{Unit, ModifiedUnit};

pub fn find(conn: &PgConnection) -> Result<Vec<Unit>, Error> {
    use crate::schema::units;

    find_all!(units::table, conn)
}

pub fn get(conn: &PgConnection, id: i32)
           -> Result<Unit, Error> {
    use crate::schema::units;

    get_one!(units::table, conn, id)
}

pub fn create(conn: &PgConnection, unit: ModifiedUnit)
              -> Result<Unit, Error> {
    use crate::schema::units;

    create!(units::table, conn, &unit)
}

pub fn update(conn: &PgConnection, id: i32, unit: ModifiedUnit)
              -> Result<Unit, Error> {
    use crate::schema::units;

    update!(units::table, conn, id, &unit)
}

pub fn delete(conn: &PgConnection, id: i32) -> Result<usize, Error> {
    use crate::schema::units;

    delete!(units::table, conn, id)
}