use crate::schema::products;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct Product {
    pub id: i32,
    pub short_name: Option<String>,
    pub full_name: Option<String>,
    pub description: Option<String>,
    pub vendor_code: Option<BigDecimal>,
    pub purchase_price: Option<BigDecimal>,
    pub selling_price: Option<BigDecimal>,
    pub default_currency: Option<i32>,
    pub default_unit: Option<i32>,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[table_name = "products"]
pub struct ModifiedProduct {
    pub short_name: Option<String>,
    pub full_name: Option<String>,
    pub description: Option<String>,
    pub vendor_code: Option<BigDecimal>,
    pub purchase_price: Option<BigDecimal>,
    pub selling_price: Option<BigDecimal>,
    pub default_currency: Option<i32>,
    pub default_unit: Option<i32>,
}