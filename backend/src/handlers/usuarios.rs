// src/handlers/usuarios.rs

use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::models::{Usuario, NewUsuario, Pool};
use crate::schema::usuarios::dsl::*;
use serde::{Deserialize, Serialize};
use bcrypt::{hash, DEFAULT_COST};
use chrono::NaiveDate;

#[derive(Deserialize, Serialize)]
pub struct UsuarioInput {
    pub nombre: String,
    pub apellido1: String,
    pub apellido2: Option<String>,
    pub email: String,
    pub telefono: Option<String>,
    pub fechanacimiento: String,
    pub dni: String,
    pub password: String,
    pub imagen_url: Option<String>,
    pub rol_id: i32,
    pub obs: Option<String>,
    pub usuario_creacion: String,
    pub usuario_modificacion: String,
}

pub async fn create_usuario(
    pool: web::Data<Pool>,
    item: web::Json<UsuarioInput>,
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    let hashed_password = hash(&item.password, DEFAULT_COST).expect("Error hashing password");

    let fecha_nacimiento_parsed = NaiveDate::parse_from_str(&item.fechanacimiento, "%Y-%m-%d")
        .expect("Invalid date format");

    let new_usuario = NewUsuario {
        nombre: &item.nombre,
        apellido1: &item.apellido1,
        apellido2: item.apellido2.as_deref(),
        email: &item.email,
        telefono: item.telefono.as_deref(),
        fechanacimiento: fecha_nacimiento_parsed,
        dni: &item.dni,
        password: &hashed_password,
        imagen_url: item.imagen_url.as_deref(),
        rol_id: item.rol_id,
        obs: item.obs.as_deref(),
        usuario_creacion: &item.usuario_creacion,
        usuario_modificacion: &item.usuario_modificacion,
    };

    diesel::insert_into(usuarios)
        .values(&new_usuario)
        .execute(&mut conn)
        .expect("Error inserting new usuario");

    HttpResponse::Ok().json(new_usuario)
}

pub async fn get_usuarios(pool: web::Data<Pool>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");
    let results = usuarios
        .load::<Usuario>(&mut conn)
        .expect("Error loading usuarios");

    HttpResponse::Ok().json(results)
}

pub async fn get_usuario_by_id(
    pool: web::Data<Pool>,
    usuario_id: web::Path<i32>,
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");
    let result = usuarios
        .filter(id.eq(usuario_id.into_inner()))
        .first::<Usuario>(&mut conn)
        .optional()
        .expect("Error loading usuario");

    if let Some(usuario) = result {
        HttpResponse::Ok().json(usuario)
    } else {
        HttpResponse::NotFound().body("Usuario not found")
    }
}

pub async fn update_usuario(
    pool: web::Data<Pool>,
    usuario_id: web::Path<i32>,
    item: web::Json<UsuarioInput>,
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    let hashed_password = hash(&item.password, DEFAULT_COST).expect("Error hashing password");

    let fecha_nacimiento_parsed = NaiveDate::parse_from_str(&item.fechanacimiento, "%Y-%m-%d")
        .expect("Invalid date format");

    let target = usuarios.filter(id.eq(usuario_id.into_inner()));
    let updated_usuario = diesel::update(target)
        .set((
            nombre.eq(&item.nombre),
            apellido1.eq(&item.apellido1),
            apellido2.eq(item.apellido2.as_deref()),
            email.eq(&item.email),
            telefono.eq(item.telefono.as_deref()),
            fechanacimiento.eq(fecha_nacimiento_parsed),
            dni.eq(&item.dni),
            password.eq(&hashed_password),
            imagen_url.eq(item.imagen_url.as_deref()),
            rol_id.eq(item.rol_id),
            obs.eq(item.obs.as_deref()),
            usuario_modificacion.eq(&item.usuario_modificacion),
            fecha_modificacion.eq(chrono::Local::now().naive_local()),
        ))
        .execute(&mut conn)
        .expect("Error updating usuario");

    if updated_usuario == 0 {
        HttpResponse::NotFound().body("Usuario not found")
    } else {
        HttpResponse::Ok().json("Usuario updated")
    }
}

pub async fn delete_usuario(
    pool: web::Data<Pool>,
    usuario_id: web::Path<i32>,
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    let deleted_usuario = diesel::delete(usuarios.filter(id.eq(usuario_id.into_inner())))
        .execute(&mut conn)
        .expect("Error deleting usuario");

    if deleted_usuario == 0 {
        HttpResponse::NotFound().body("Usuario not found")
    } else {
        HttpResponse::Ok().json("Usuario deleted")
    }
}
