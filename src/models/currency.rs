use crate::schema::currencies;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct Currency {
    pub id: i32,
    pub name: String,
    pub short_name: Option<String>,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[table_name = "currencies"]
pub struct ModifiedCurrency {
    pub name: Option<String>,
    pub short_name: Option<String>,
}