use actix_web::{HttpResponse, Result, web};

use crate::AppState;
use crate::repository::document_repository;
use crate::models::document::ModifiedDocument;

pub async fn find(data: web::Data<AppState>) -> Result<HttpResponse> {
    let conn = data.pool.get().expect("failed to get connection");
    let result = web::block(move || document_repository::find(&conn))
        .await;

    match result {
        Ok(vec) => Ok(HttpResponse::Ok().json(vec)),
        Err(_) => Ok(HttpResponse::InternalServerError().finish())
    }
}

pub async fn get(data: web::Data<AppState>,
                 path: web::Path<(i32,)>) -> Result<HttpResponse> {
    let conn = data.pool.get().expect("failed to get connection");
    let result = web::block(move || document_repository::get(&conn, path.into_inner().0))
        .await;

    match result {
        Ok(role) => Ok(HttpResponse::Ok().json(role)),
        Err(_) => Ok(HttpResponse::InternalServerError().finish())
    }
}

pub async fn create(data: web::Data<AppState>,
                    role: web::Json<ModifiedDocument>) -> Result<HttpResponse> {
    let conn = data.pool.get().expect("failed to get connection");
    let result = web::block(move || document_repository::create(&conn, role.0))
        .await;

    match result {
        Ok(role) => Ok(HttpResponse::Ok().json(role)),
        Err(_) => Ok(HttpResponse::InternalServerError().finish())
    }
}

pub async fn update(data: web::Data<AppState>,
                    role: web::Json<ModifiedDocument>,
                    path: web::Path<(i32,)>) -> Result<HttpResponse> {
    let conn = data.pool.get().expect("failed to get connection");
    let result = web::block(move || document_repository::update(&conn, path.into_inner().0, role.0))
        .await;

    match result {
        Ok(role) => Ok(HttpResponse::Ok().json(role)),
        Err(_) => Ok(HttpResponse::InternalServerError().finish())
    }
}

pub async fn delete(data: web::Data<AppState>,
                    path: web::Path<(i32,)>) -> Result<HttpResponse> {
    let conn = data.pool.get().expect("failed to get connection");
    let result = web::block(move || document_repository::delete(&conn, path.into_inner().0))
        .await;

    match result {
        Ok(num_deleted) => Ok(HttpResponse::Ok().json(json!({"num_deleted": num_deleted}))),
        Err(_) => Ok(HttpResponse::InternalServerError().finish())
    }
}