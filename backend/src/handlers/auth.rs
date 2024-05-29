// src/handlers/auth.rs

use actix_web::{web, HttpResponse, Responder};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Deserialize)]
pub struct AuthData {
    username: String,
    password: String,
}

pub async fn login(auth_data: web::Json<AuthData>) -> impl Responder {
    // Validar el usuario y la contraseña (esto es solo un ejemplo simple)
    if auth_data.username == "admin" && auth_data.password == "password" {
        let claims = Claims {
            sub: auth_data.username.clone(),
            exp: 10000000000,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("secret".as_ref()),
        )
        .unwrap();

        HttpResponse::Ok().json(token)
    } else {
        HttpResponse::Unauthorized().body("Invalid username or password")
    }
}
