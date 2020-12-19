use crate::schema::documents;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::models::document_item::DocumentItem;

#[derive(Queryable, Serialize)]
pub struct Document {
    pub id: i32,
    pub created_at: Option<NaiveDateTime>,
    pub edited_at: Option<NaiveDateTime>,
    pub is_valid: Option<bool>,
    pub sender_company_id: Option<i32>,
    pub middleman_company_id: Option<i32>,
    pub recipient_company_id: Option<i32>,
    pub document_type_id: Option<i32>,
    pub document_number: Option<i32>,
    pub comment: Option<String>,
}

#[derive(AsChangeset, Insertable, Deserialize)]
#[table_name = "documents"]
pub struct ModifiedDocument {
    pub edited_at: Option<NaiveDateTime>,
    pub is_valid: Option<bool>,
    pub sender_company_id: Option<i32>,
    pub middleman_company_id: Option<i32>,
    pub recipient_company_id: Option<i32>,
    pub document_type_id: Option<i32>,
    pub document_number: Option<i32>,
    pub comment: Option<String>,
}

#[derive(Queryable, Serialize)]
pub struct GetOneDocument {
    pub id: i32,
    pub is_valid: Option<bool>,
    pub sender_company_id: Option<i32>,
    pub middleman_company_id: Option<i32>,
    pub recipient_company_id: Option<i32>,
    pub document_type_id: Option<i32>,
    pub document_number: Option<i32>,
    pub comment: Option<String>,
    pub document_items: Vec<DocumentItem>
}