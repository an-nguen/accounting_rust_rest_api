use diesel::prelude::*;
use diesel::result::Error;
use crate::models::document_type::{DocumentType, ModifiedDocumentType};

pub fn find(conn: &PgConnection) -> Result<Vec<DocumentType>, Error> {
    use crate::schema::document_types;

    find_all!(document_types::table, conn)
}

pub fn get(conn: &PgConnection, document_type_id: i32)
           -> Result<DocumentType, Error> {
    use crate::schema::document_types;

    get_one!(document_types::table, conn, document_type_id)
}

pub fn create(conn: &PgConnection, document_type: ModifiedDocumentType)
              -> Result<DocumentType, Error> {
    use crate::schema::document_types;

    create!(document_types::table, conn, &document_type)
}

pub fn update(conn: &PgConnection, document_type_id: i32, document_type: ModifiedDocumentType)
              -> Result<DocumentType, Error> {
    use crate::schema::document_types;

    update!(document_types::table, conn, document_type_id, &document_type)
}

pub fn delete(conn: &PgConnection, document_type_id: i32) -> Result<usize, Error> {
    use crate::schema::document_types;

    delete!(document_types::table, conn, document_type_id)
}