use crate::schema::roles_privileges;
use crate::models::role::Role;
use crate::models::privilege::Privilege;

#[derive(Queryable, Insertable, Associations)]
#[belongs_to(Role)]
#[belongs_to(Privilege)]
#[table_name="roles_privileges"]
pub struct RolePrivilege {
    pub role_id: i32,
    pub privilege_id: i32
}
