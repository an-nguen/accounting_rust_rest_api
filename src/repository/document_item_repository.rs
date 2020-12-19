use diesel::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

use crate::models::document_item::{DocumentItem, ModifiedDocumentItem};

pub fn find(conn: &PgConnection) -> Result<Vec<DocumentItem>, Error> {
    use crate::schema::document_items;

    find_all!(document_items::table, conn)
}

pub fn get(conn: &PgConnection, id: i32)
           -> Result<DocumentItem, Error> {
    use crate::schema::document_items;

    get_one!(document_items::table, conn, id)
}

pub fn create(conn: &PgConnection, document_item: ModifiedDocumentItem)
              -> Result<DocumentItem, Error> {
    use crate::schema::document_items;

    create!(document_items::table, conn, &document_item)
}

pub fn update(conn: &PgConnection, id: i32, document_item: ModifiedDocumentItem)
              -> Result<DocumentItem, Error> {
    use crate::schema::document_items;

    update!(document_items::table, conn, id, &document_item)
}

pub fn delete(conn: &PgConnection, id: i32) -> Result<usize, Error> {
    use crate::schema::document_items;

    delete!(document_items::table, conn, id)
}