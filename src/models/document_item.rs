use serde::{Deserialize, Serialize};
use crate::schema::document_items;
use bigdecimal::BigDecimal;

#[derive(Queryable, Serialize)]
pub struct DocumentItem {
    pub id: i32,
    pub product_id: Option<i32>,
    pub document_id: Option<i32>,
    pub series_code: Option<BigDecimal>,
    pub quantity:  BigDecimal,
    pub price: BigDecimal,
    pub total: BigDecimal,
    pub currency_id: Option<i32>,
    pub unit_id: Option<i32>,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[table_name = "document_items"]
pub struct ModifiedDocumentItem {
    pub product_id: Option<i32>,
    pub document_id: Option<i32>,
    pub series_code: Option<BigDecimal>,
    pub quantity:  BigDecimal,
    pub price: BigDecimal,
    pub total: BigDecimal,
    pub currency_id: Option<i32>,
    pub unit_id: Option<i32>,
}