// src/main.rs

use actix_web::{web, App, HttpServer, Responder};
use std::io::Result;
use crate::config::Config; // Asegúrate de que el path sea correcto según tu estructura de módulos

mod config;

async fn greet() -> impl Responder {
    "Hello, world!"
}

#[actix_web::main]
async fn main() -> Result<()> {
    let config = Config::new();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
    })
    .bind(format!("0.0.0.0:{}", config.server_port))?
    .run()
    .await
}
