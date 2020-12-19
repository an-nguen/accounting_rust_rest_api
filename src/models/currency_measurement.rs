use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::schema::currencies_measurements;
use bigdecimal::BigDecimal;

#[derive(Queryable, Serialize)]
pub struct CurrencyMeasurement {
    pub id: i32,
    pub currency_id: Option<i32>,
    pub value: BigDecimal,
    pub date: Option<NaiveDate>,
}

#[derive(AsChangeset, Insertable, Deserialize)]
#[table_name = "currencies_measurements"]
pub struct ModifiedCurrencyMeasurement {
    pub currency_id: Option<i32>,
    pub value: Option<BigDecimal>,
    pub date: Option<NaiveDate>,
}