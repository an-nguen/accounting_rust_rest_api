use serde::{Deserialize, Serialize};
use crate::schema::units;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Unit {
    pub id: i32,
    pub name: String,
    pub short_name: Option<String>
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[table_name = "units"]
pub struct ModifiedUnit {
    pub name: Option<String>,
    pub short_name: Option<String>
}