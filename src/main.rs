#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate serde_json;

use std::env;
use env_logger::Env;
use actix_web::{App, HttpServer, middleware::Logger, web};
use diesel::{pg::PgConnection,
             r2d2::{self, ConnectionManager, Pool}};
use dotenv::dotenv;

#[cfg(test)]
mod test;

pub mod utils;
pub mod schema;
pub mod models;
pub mod repository;
pub mod controllers;
pub mod middlewares;

pub struct AppState {
    pub pool: Pool<ConnectionManager<PgConnection>>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let database_manager = ConnectionManager::<PgConnection>::new(database_url);
    let database_pool = r2d2::Pool::builder().build(database_manager)
        .expect("Failed to create pool.");
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new (move || {
        let state = AppState {
            pool: database_pool.clone()
        };
        App::new()
            .data(state)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .configure(routes)
    }).workers(4)
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

fn routes(app: &mut web::ServiceConfig) {
    app
        .service(web::scope("/api")
            .service(web::resource("users/login").route(web::post().to(controllers::user_controller::login)))
            .service(web::resource("users").wrap(middlewares::auth::JwtAuth)
                .route(web::get().to(controllers::user_controller::find))
                .route(web::post().to(controllers::user_controller::create))
            )
            .service(web::resource("users/{id}").wrap(middlewares::auth::JwtAuth)
                .route(web::get().to(controllers::user_controller::get))
                .route(web::put().to(controllers::user_controller::update))
                .route(web::delete().to(controllers::user_controller::delete))
            )
            .service(web::resource("roles").wrap(middlewares::auth::JwtAuth)
                .route(web::get().to(controllers::role_controller::find))
                .route(web::post().to(controllers::role_controller::create))
            )
            .service(web::resource("roles/{id}").wrap(middlewares::auth::JwtAuth)
                .route(web::get().to(controllers::role_controller::get))
                .route(web::put().to(controllers::role_controller::update))
                .route(web::delete().to(controllers::role_controller::delete))
            )
            .service(web::resource("privileges").wrap(middlewares::auth::JwtAuth)
                .route(web::get().to(controllers::privilege_controller::find))
                .route(web::post().to(controllers::privilege_controller::create))
            )
            .service(web::resource("privileges/{id}").wrap(middlewares::auth::JwtAuth)
                .route(web::get().to(controllers::privilege_controller::get))
                .route(web::put().to(controllers::privilege_controller::update))
                .route(web::delete().to(controllers::privilege_controller::delete))
            )
            .service(web::resource("companies").wrap(middlewares::auth::JwtAuth)
                .route(web::get().to(controllers::company_controller::find))
                .route(web::post().to(controllers::company_controller::create))
            )
            .service(web::resource("companies/{id}").wrap(middlewares::auth::JwtAuth)
                .route(web::get().to(controllers::company_controller::get))
                .route(web::put().to(controllers::company_controller::update))
                .route(web::delete().to(controllers::company_controller::delete))
            )
            .service(web::resource("documents").wrap(middlewares::auth::JwtAuth)
                .route(web::get().to(controllers::document_controller::find))
                .route(web::post().to(controllers::document_controller::create))
            )
            .service(web::resource("documents/{id}").wrap(middlewares::auth::JwtAuth)
                .route(web::get().to(controllers::document_controller::get))
                .route(web::put().to(controllers::document_controller::update))
                .route(web::delete().to(controllers::document_controller::delete))
            )
            .service(web::resource("units").wrap(middlewares::auth::JwtAuth)
                .route(web::get().to(controllers::unit_controller::find))
                .route(web::post().to(controllers::unit_controller::create))
            )
            .service(web::resource("units/{id}").wrap(middlewares::auth::JwtAuth)
                .route(web::get().to(controllers::unit_controller::get))
                .route(web::put().to(controllers::unit_controller::update))
                .route(web::delete().to(controllers::unit_controller::delete))
            )
            .service(web::resource("document_types").wrap(middlewares::auth::JwtAuth)
                .route(web::get().to(controllers::document_type_controller::find))
                .route(web::post().to(controllers::document_type_controller::create))
            )
            .service(web::resource("document_types/{id}").wrap(middlewares::auth::JwtAuth)
                .route(web::get().to(controllers::document_type_controller::get))
                .route(web::put().to(controllers::document_type_controller::update))
                .route(web::delete().to(controllers::document_type_controller::delete))
            )
            .service(web::resource("currencies").wrap(middlewares::auth::JwtAuth)
                .route(web::get().to(controllers::currency_controller::find))
                .route(web::post().to(controllers::currency_controller::create))
            )
            .service(web::resource("currencies/{id}").wrap(middlewares::auth::JwtAuth)
                .route(web::get().to(controllers::currency_controller::get))
                .route(web::put().to(controllers::currency_controller::update))
                .route(web::delete().to(controllers::currency_controller::delete))
            )
        );
}