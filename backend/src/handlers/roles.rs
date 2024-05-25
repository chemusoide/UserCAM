// src/handlers/roles.rs

use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::models::{Rol, NewRol, Pool};
use crate::schema::roles::dsl::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RolInput {
    pub nombre: String,
    pub obs: Option<String>,
    pub usuario_creacion: String,
    pub usuario_modificacion: String,
}

pub async fn create_rol(
    pool: web::Data<Pool>,
    item: web::Json<RolInput>,
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    let new_rol = NewRol {
        nombre: &item.nombre,
        obs: item.obs.as_deref(),
        usuario_creacion: &item.usuario_creacion,
        usuario_modificacion: &item.usuario_modificacion,
    };

    diesel::insert_into(roles)
        .values(&new_rol)
        .execute(&mut conn)
        .expect("Error inserting new rol");

    HttpResponse::Ok().json(new_rol)
}

pub async fn get_roles(pool: web::Data<Pool>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");
    let results = roles
        .load::<Rol>(&mut conn)
        .expect("Error loading roles");

    HttpResponse::Ok().json(results)
}

pub async fn get_rol_by_id(
    pool: web::Data<Pool>,
    rol_id: web::Path<i32>,
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");
    let result = roles
        .filter(id.eq(rol_id.into_inner()))
        .first::<Rol>(&mut conn)
        .optional()
        .expect("Error loading rol");

    if let Some(rol) = result {
        HttpResponse::Ok().json(rol)
    } else {
        HttpResponse::NotFound().body("Rol not found")
    }
}

pub async fn update_rol(
    pool: web::Data<Pool>,
    rol_id: web::Path<i32>,
    item: web::Json<RolInput>,
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    let target = roles.filter(id.eq(rol_id.into_inner()));
    let updated_rol = diesel::update(target)
        .set((
            nombre.eq(&item.nombre),
            obs.eq(item.obs.as_deref()),
            usuario_modificacion.eq(&item.usuario_modificacion),
            fecha_modificacion.eq(chrono::Local::now().naive_local()),
        ))
        .execute(&mut conn)
        .expect("Error updating rol");

    if updated_rol == 0 {
        HttpResponse::NotFound().body("Rol not found")
    } else {
        HttpResponse::Ok().json("Rol updated")
    }
}

pub async fn delete_rol(
    pool: web::Data<Pool>,
    rol_id: web::Path<i32>,
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    let deleted_rol = diesel::delete(roles.filter(id.eq(rol_id.into_inner())))
        .execute(&mut conn)
        .expect("Error deleting rol");

    if deleted_rol == 0 {
        HttpResponse::NotFound().body("Rol not found")
    } else {
        HttpResponse::Ok().json("Rol deleted")
    }
}
