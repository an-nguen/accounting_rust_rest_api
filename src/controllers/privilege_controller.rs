use actix_web::{HttpResponse, Result};
use actix_web::web::{block, Data, Json, Path};

use crate::AppState;
use crate::models::privilege::ModifiedPrivilege;
use crate::repository::privilege_repository;

pub async fn find(data: Data<AppState>) -> Result<HttpResponse> {
    let conn = data.pool.get().expect("failed to get connection");
    let result = block(move || privilege_repository::find(&conn))
        .await;

    match result {
        Ok(vec) => Ok(HttpResponse::Ok().json(vec)),
        Err(_) => Ok(HttpResponse::InternalServerError().finish())
    }
}

pub async fn get(data: Data<AppState>, path: Path<(i32,)>) -> Result<HttpResponse> {
    let conn = data.pool.get().expect("failed to get connection");
    let result = block(move || privilege_repository::get(&conn, path.into_inner().0))
        .await;

    match result {
        Ok(privilege) => Ok(HttpResponse::Ok().json(privilege)),
        Err(_) => Ok(HttpResponse::InternalServerError().finish())
    }
}

pub async fn create(data: Data<AppState>, privilege: Json<ModifiedPrivilege>) -> Result<HttpResponse> {
    let conn = data.pool.get().expect("failed to get connection");
    let result = block(move || privilege_repository::create(&conn, privilege.0))
        .await;

    match result {
        Ok(privilege) => Ok(HttpResponse::Ok().json(privilege)),
        Err(_) => Ok(HttpResponse::InternalServerError().finish())
    }
}

pub async fn update(data: Data<AppState>, path: Path<(i32,)>, privilege: Json<ModifiedPrivilege>) -> Result<HttpResponse> {
    let conn = data.pool.get().expect("failed to get connection");
    let result = block(move ||
        privilege_repository::update(&conn, path.into_inner().0, privilege.0))
        .await;

    match result {
        Ok(privilege) => Ok(HttpResponse::Ok().json(privilege)),
        Err(_) => Ok(HttpResponse::InternalServerError().finish())
    }
}

pub async fn delete(data: Data<AppState>, path: Path<(i32,)>) -> Result<HttpResponse> {
    let conn = data.pool.get().expect("failed to get connection");
    let result = block(move ||
        privilege_repository::delete(&conn, path.into_inner().0))
        .await;

    match result {
        Ok(num_deleted) => Ok(HttpResponse::Ok().json(json!({"num_deleted": num_deleted}))),
        Err(_) => Ok(HttpResponse::InternalServerError().finish())
    }
}