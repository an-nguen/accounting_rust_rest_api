use crate::schema::document_types;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct DocumentType {
    pub id: i32,
    pub name: Option<String>
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[table_name = "document_types"]
pub struct ModifiedDocumentType {
    pub name: Option<String>
}