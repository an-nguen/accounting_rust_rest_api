use serde::{Deserialize, Serialize};
use crate::schema::companies;

#[derive(Queryable, Serialize)]
pub struct Company {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub address: Option<String>,
    pub itn: Option<String>,
    pub okonkh: Option<String>,
    pub okpo: Option<String>,
    pub bic: Option<String>,
    pub bank_account: Option<String>,
}

#[derive(AsChangeset, Insertable, Deserialize)]
#[table_name = "companies"]
pub struct ModifiedCompany {
    pub name: Option<String>,
    pub description: Option<String>,
    pub address: Option<String>,
    pub itn: Option<String>,
    pub okonkh: Option<String>,
    pub okpo: Option<String>,
    pub bic: Option<String>,
    pub bank_account: Option<String>,
}

