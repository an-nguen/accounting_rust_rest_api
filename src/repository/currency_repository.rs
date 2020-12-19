use diesel::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

use crate::models::currency::{Currency, ModifiedCurrency};

pub fn find(conn: &PgConnection) -> Result<Vec<Currency>, Error> {
    use crate::schema::currencies;

    find_all!(currencies::table, conn)
}

pub fn get(conn: &PgConnection, id: i32)
           -> Result<Currency, Error> {
    use crate::schema::currencies;

    get_one!(currencies::table, conn, id)
}

pub fn create(conn: &PgConnection, document_item: ModifiedCurrency)
              -> Result<Currency, Error> {
    use crate::schema::currencies;

    create!(currencies::table, conn, &document_item)
}

pub fn update(conn: &PgConnection, id: i32, document_item: ModifiedCurrency)
              -> Result<Currency, Error> {
    use crate::schema::currencies;

    update!(currencies::table, conn, id, &document_item)
}

pub fn delete(conn: &PgConnection, id: i32) -> Result<usize, Error> {
    use crate::schema::currencies;

    delete!(currencies::table, conn, id)
}