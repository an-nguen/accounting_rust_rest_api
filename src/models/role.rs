use crate::schema::roles;
use crate::models::privilege::Privilege;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Role {
    pub id: i32,
    pub name: Option<String>
}

#[derive(Deserialize)]
pub struct ReqModifiedRole {
    pub name: Option<String>,
    pub privileges: Vec<Privilege>
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[table_name = "roles"]
pub struct ModifiedRole {
    pub name: Option<String>,
}