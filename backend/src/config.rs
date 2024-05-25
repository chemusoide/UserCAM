// src/config.rs

use std::env;
use dotenv::dotenv;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{self, ConnectionManager};

pub struct Config {
    pub database_url: String,
    pub server_port: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();
        let username = env::var("MYSQL_USER").expect("MYSQL_USER must be set");
        let password = env::var("MYSQL_PASSWORD").expect("MYSQL_PASSWORD must be set");
        let database = env::var("MYSQL_DATABASE").expect("MYSQL_DATABASE must be set");
        let host = env::var("MYSQL_HOST").unwrap_or_else(|_| "mysql".to_string());

        let database_url = format!("mysql://{}:{}@{}/{}", username, password, host, database);
        let server_port = env::var("SERVER_PORT").unwrap_or_else(|_| "8000".to_string());

        Config {
            database_url,
            server_port,
        }
    }

    pub fn create_db_pool(&self) -> r2d2::Pool<ConnectionManager<MysqlConnection>> {
        let manager = ConnectionManager::<MysqlConnection>::new(&self.database_url);
        r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.")
    }
}