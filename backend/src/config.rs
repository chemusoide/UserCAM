// src/config.rs

use dotenv::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
    pub server_port: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok(); // Carga las variables de entorno desde el archivo .env
        
        let username = env::var("MYSQL_USER").expect("MYSQL_USER must be set");
        let password = env::var("MYSQL_PASSWORD").expect("MYSQL_PASSWORD must be set");
        let database = env::var("MYSQL_DATABASE").expect("MYSQL_DATABASE must be set");
        let host = env::var("MYSQL_HOST").unwrap_or_else(|_| "mysql".to_string()); // Asigna 'mysql' por defecto si MYSQL_HOST no está configurado

        let database_url = format!("mysql://{}:{}@{}/{}", username, password, host, database);
        let server_port = env::var("SERVER_PORT").unwrap_or_else(|_| "8000".to_string());

        Config {
            database_url,
            server_port,
        }
    }
}
