use chrono::Utc;
use diesel::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

use crate::models::document::{Document, GetOneDocument, ModifiedDocument};
use crate::models::document_item::DocumentItem;

fn get_one_document(document: &Document, document_item_vec: Vec<DocumentItem>)
    -> Result<GetOneDocument, Error> {
    let resp_document = GetOneDocument {
        id: document.id,
        is_valid: document.is_valid,
        sender_company_id: document.sender_company_id,
        middleman_company_id: document.middleman_company_id,
        recipient_company_id: document.recipient_company_id,
        document_type_id: document.document_type_id,
        document_number: document.document_number,
        comment: document.comment.clone(),
        document_items: document_item_vec
    };

    Ok(resp_document)
}

pub fn find(conn: &PgConnection) -> Result<Vec<Document>, Error> {
    use crate::schema::documents;

    find_all!(documents::table, conn)
}

pub fn get(conn: &PgConnection, req_id: i32)
           -> Result<GetOneDocument, Error> {
    use crate::schema::documents;
    use crate::schema::document_items;

    let document_item_vec = document_items::table.filter(document_items::document_id.eq(req_id))
        .load::<DocumentItem>(conn)?;
    let document = documents::table
        .find(req_id)
        .first(conn)?;
    get_one_document(&document, document_item_vec)
}

pub fn create(conn: &PgConnection, document: ModifiedDocument)
              -> Result<GetOneDocument, Error> {
    use crate::schema::documents;
    use crate::schema::document_items;

    let new_document: Document = create!(documents::table, conn, &document)?;
    let document_item_vec = document_items::table
        .filter(document_items::document_id.eq(new_document.id))
        .load::<DocumentItem>(conn)?;
    get_one_document(&new_document, document_item_vec)
}

pub fn update(conn: &PgConnection, id: i32, mut document: ModifiedDocument)
              -> Result<GetOneDocument, Error> {
    use crate::schema::documents;
    use crate::schema::document_items;

    document.edited_at = Option::from(Utc::now().naive_utc());
    let upd_document: Document = update!(documents::table, conn, id, &document)?;
    let document_item_vec = document_items::table
        .filter(document_items::document_id.eq(upd_document.id))
        .load::<DocumentItem>(conn)?;
    get_one_document(&upd_document, document_item_vec)
}

pub fn delete(conn: &PgConnection, id: i32) -> Result<usize, Error> {
    use crate::schema::documents;

    delete!(documents::table, conn, id)
}