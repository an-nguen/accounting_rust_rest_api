use diesel::prelude::*;
use jsonwebtoken::errors::{Error, ErrorKind};

use crate::models::role::Role;
use crate::models::user::{CreateUser, Credentials, GetOneUser, ReqNewUser, ReqUpdUser, UpdateUser, User};
use crate::repository::user_role_repository;
use crate::utils::argon2_hash::{hash_password, verify_password};
use crate::utils::jwt::GenerateJwtToken;

pub fn find(conn: &PgConnection) -> Result<Vec<User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let mut user_vec = users.load::<User>(conn)?;
    user_vec.iter_mut().for_each(|user| {
        user.password = String::from("");
    });

    Ok(user_vec)
}

pub fn get(conn: &PgConnection, user_id: i32) -> Result<GetOneUser, diesel::result::Error> {
    use crate::schema::users;
    use crate::schema::roles;
    use crate::schema::users_roles;

    let (id, username, email, first_name, last_name, birthday) = users::table
        .find(user_id)
        .select((users::id, users::username, users::email, users::first_name, users::last_name, users::birthday))
        .first(conn)?;
    let role_vec = users::table
        .inner_join(users_roles::table.inner_join(roles::table))
        .filter(users::id.eq(user_id))
        .select(roles::all_columns)
        .load::<Role>(conn)?;
    let resp_user = GetOneUser {
        id,
        username,
        email,
        first_name,
        last_name,
        birthday,
        roles: role_vec
    };

    Ok(resp_user)
}

pub fn create(conn: &PgConnection, req_new_user: ReqNewUser) -> Result<GetOneUser, diesel::result::Error> {
    use crate::schema::users;
    use crate::schema::roles::{id, name};
    use crate::schema::roles::dsl::*;
    use crate::schema::users_roles::dsl::*;

    let new_user = CreateUser {
        username: req_new_user.username,
        password: hash_password(&req_new_user.password),
        email: req_new_user.email,
        first_name: req_new_user.first_name,
        last_name: req_new_user.last_name,
        birthday: req_new_user.birthday
    };
    let user = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(conn)?;
    user_role_repository::create(conn, user.id, req_new_user.roles);

    let resp_roles = roles.inner_join(users_roles.on(user_id.eq(user.id)))
        .select((id, name)).load::<Role>(conn)?;
    let resp_user = GetOneUser {
        id: user.id,
        username: user.username,
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
        birthday: user.birthday,
        roles: resp_roles
    };

    Ok(resp_user)
}

pub fn update(conn: &PgConnection, user_id: i32, req_upd_user: ReqUpdUser) -> Result<GetOneUser, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    use crate::schema::roles;
    use crate::schema::users_roles;

    let upd_user = UpdateUser {
        password: Option::from(hash_password(&req_upd_user.password.unwrap())),
        email: req_upd_user.email,
        first_name: req_upd_user.first_name,
        last_name:req_upd_user.last_name,
        birthday: req_upd_user.birthday
    };
    let user = diesel::update(users.find(user_id))
        .set(&upd_user).get_result::<User>(conn)
        .expect("failed to update user");
    user_role_repository::delete(conn, user.id).expect("failed to delete users_roles");
    user_role_repository::create(conn, user.id, req_upd_user.roles);
    let resp_roles = roles::dsl::roles.inner_join(users_roles::dsl::users_roles.on(users_roles::user_id.eq(&user.id)))
        .select((roles::id, roles::name)).load::<Role>(conn)?;
    let resp_user = GetOneUser {
        id: user.id,
        username: user.username,
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
        birthday: user.birthday,
        roles: resp_roles
    };

    Ok(resp_user)
}

pub fn delete(conn: &PgConnection, user_id: i32) -> Result<usize, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    user_role_repository::delete(conn, user_id).expect("failed to delete user_role");
    diesel::delete(users.find(user_id)).execute(conn)
}

pub fn login(conn: &PgConnection, credentials: Credentials) -> Result<String, Error> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(username.eq(credentials.username))
        .first::<User>(conn);
    match user {
        Ok(user) => {
            let token = if verify_password(&credentials.password, &user.password) {
                user.generate_token()
            } else { Result::Ok(String::from("")) };
            token
        },
        Err(_) => Err(jsonwebtoken::errors::Error::from(ErrorKind::InvalidSubject))
    }

}