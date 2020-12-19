use chrono::NaiveDate;
use diesel::PgConnection;
use serde::{Deserialize, Serialize};

use crate::schema::users;
use crate::utils::jwt::Claims;
use crate::models::role::Role;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub birthday: Option<NaiveDate>,
    pub valid_to: Option<NaiveDate>
}

#[derive(Queryable, Serialize)]
pub struct GetUser {
    pub id: i32,
    pub username: String,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub birthday: Option<NaiveDate>,
}

#[derive(Queryable, Serialize)]
pub struct GetOneUser {
    pub id: i32,
    pub username: String,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub birthday: Option<NaiveDate>,
    pub roles: Vec<Role>
}

impl User {
    pub fn is_valid_token_claim(token_claim: &Claims, conn: &PgConnection) -> bool {
        use diesel::prelude::*;
        use crate::schema::users::dsl::*;

        users.filter(username.eq(&token_claim.sub))
            .get_result::<User>(conn)
            .is_ok()
    }
}

#[derive(Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct ReqNewUser {
    pub username: String,
    pub password: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub birthday: NaiveDate,
    pub roles: Vec<Role>
}

#[derive(Debug, Insertable, Deserialize)]
#[table_name = "users"]
pub struct CreateUser {
    pub username: String,
    pub password: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub birthday: NaiveDate
}

#[derive(Deserialize)]
pub struct ReqUpdUser {
    pub password: Option<String>,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub birthday: Option<NaiveDate>,
    pub roles: Vec<Role>
}

#[derive(AsChangeset, Deserialize)]
#[table_name = "users"]
pub struct UpdateUser {
    pub password: Option<String>,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub birthday: Option<NaiveDate>
}
