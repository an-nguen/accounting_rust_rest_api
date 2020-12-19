use crate::models::user::User;
use crate::models::role::Role;
use crate::schema::users_roles;

#[derive(Queryable, Debug, Insertable, Associations)]
#[belongs_to(User)]
#[belongs_to(Role)]
#[table_name="users_roles"]
pub struct UserRole {
    pub user_id: i32,
    pub role_id: i32
}