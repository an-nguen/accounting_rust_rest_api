use crate::schema::privileges;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Privilege {
    pub id: i32,
    pub name: String
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[table_name = "privileges"]
pub struct ModifiedPrivilege {
    pub name: String,
}