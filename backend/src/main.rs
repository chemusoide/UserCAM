// src/main.rs

use actix_web::{web, App, HttpServer, Responder};
use std::io::Result;
use crate::config::Config;
use crate::handlers::{usuarios, roles, dojos, alumnos, grados};
use env_logger::Env;
use crate::middleware::auth::Authentication;

mod config;
mod db;
mod models;
mod schema;
mod handlers;
mod middleware;

async fn greet() -> impl Responder {
    "Hello, world!"
}

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let config = Config::new();
    let pool = config.create_db_pool();

    HttpServer::new(move || {
        App::new()
            .wrap(Authentication) 
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(greet))
            .service(
                web::scope("/usuarios")
                    .route("", web::post().to(usuarios::create_usuario))
                    .route("", web::get().to(usuarios::get_usuarios))
                    .route("/{id}", web::get().to(usuarios::get_usuario_by_id))
                    .route("/{id}", web::put().to(usuarios::update_usuario))
                    .route("/{id}", web::delete().to(usuarios::delete_usuario)),
            )
            .service(
                web::scope("/roles")
                    .route("", web::post().to(roles::create_rol))
                    .route("", web::get().to(roles::get_roles))
                    .route("/{id}", web::get().to(roles::get_rol_by_id))
                    .route("/{id}", web::put().to(roles::update_rol))
                    .route("/{id}", web::delete().to(roles::delete_rol)),
            )
            .service(
                web::scope("/dojos")
                    .route("", web::post().to(dojos::create_dojo))
                    .route("", web::get().to(dojos::get_dojos))
                    .route("/{id}", web::get().to(dojos::get_dojo_by_id))
                    .route("/{id}", web::put().to(dojos::update_dojo))
                    .route("/{id}", web::delete().to(dojos::delete_dojo)),
            )
            .service(
                web::scope("/alumnos")
                    .route("", web::post().to(alumnos::create_alumno))
                    .route("", web::get().to(alumnos::get_alumnos))
                    .route("/{id}", web::get().to(alumnos::get_alumno_by_id))
                    .route("/{id}", web::put().to(alumnos::update_alumno))
                    .route("/{id}", web::delete().to(alumnos::delete_alumno)),
            )
            .service(
                web::scope("/grados")
                    .route("", web::post().to(grados::create_grado))
                    .route("", web::get().to(grados::get_grados))
                    .route("/{id}", web::get().to(grados::get_grado_by_id))
                    .route("/{id}", web::put().to(grados::update_grado))
                    .route("/{id}", web::delete().to(grados::delete_grado)),
            )
    })
    .bind(format!("0.0.0.0:{}", config.server_port))?
    .run()
    .await
}
