use actix_web::{web, HttpResponse, Result};
use crate::AppState;
use crate::models::user::{Credentials, ReqNewUser, ReqUpdUser};

pub async fn find(data: web::Data<AppState>) -> Result<HttpResponse> {
    let conn = data.pool.get()
        .expect("failed to get connection");
    let result = web::block(move || crate::repository::user_repository::find(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    Ok(HttpResponse::Ok().json(result))
}

pub async fn create(data: web::Data<AppState>,
                    new_user: web::Json<ReqNewUser>) -> Result<HttpResponse> {
    if new_user.password.is_empty() {
        Ok(HttpResponse::BadRequest().json(json!({"error": "password cannot be null"})))
    } else {
        let conn = data.pool.get()
            .expect("failed to get connection");
        let result = web::block(move || crate::repository::user_repository::create(&conn, new_user.0))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;
        Ok(HttpResponse::Ok().json(result))
    }

}

pub async fn update(data: web::Data<AppState>,
                    path: web::Path<(i32,)>,
                    upd_user: web::Json<ReqUpdUser>) -> Result<HttpResponse> {
    let conn = data.pool.get()
        .expect("failed to get connection");
    let result = web::block(move || crate::repository::user_repository::update(&conn, path.into_inner().0,upd_user.0))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    Ok(HttpResponse::Ok().json(result))
}

pub async fn delete(data: web::Data<AppState>, path: web::Path<(i32,)>) -> Result<HttpResponse> {
    let conn = data.pool.get()
        .expect("failed to get connection");
    let result = web::block(move || crate::repository::user_repository::delete(&conn, path.into_inner().0))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    let obj = json!({"num_deleted": result});
    Ok(HttpResponse::Ok().json(obj))
}

pub async fn get(data: web::Data<AppState>, path: web::Path<(i32,)>) -> Result<HttpResponse> {
    let conn = data.pool.get()
        .expect("failed to get connection");
    let result = web::block(move || crate::repository::user_repository::get(&conn, path.into_inner().0))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    let obj = json!(result);
    Ok(HttpResponse::Ok().json(obj))
}

pub async fn login(data: web::Data<AppState>, credentials: web::Json<Credentials>) -> Result<HttpResponse> {
    let conn = data.pool.get()
        .expect("failed to get connection");
    let result = web::block(move || crate::repository::user_repository::login(&conn, credentials.0))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    let obj = json!({"token": result});
    Ok(HttpResponse::Ok().json(obj))
}