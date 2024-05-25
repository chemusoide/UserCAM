// src/handlers/grados.rs

use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::models::{Grado, NewGrado, Pool};
use crate::schema::grados::dsl::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Deserialize, Serialize)]
pub struct GradoInput {
    pub nombre: String,
    pub fecha_obtencion: String,
    pub obs: Option<String>,
    pub usuario_creacion: String,
    pub usuario_modificacion: String,
}

pub async fn create_grado(
    pool: web::Data<Pool>,
    item: web::Json<GradoInput>,
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    let fecha_obtencion_parsed = NaiveDate::parse_from_str(&item.fecha_obtencion, "%Y-%m-%d")
        .expect("Invalid date format");

    let new_grado = NewGrado {
        nombre: &item.nombre,
        fecha_obtencion: fecha_obtencion_parsed,
        obs: item.obs.as_deref(),
        usuario_creacion: &item.usuario_creacion,
        usuario_modificacion: &item.usuario_modificacion,
    };

    diesel::insert_into(grados)
        .values(&new_grado)
        .execute(&mut conn)
        .expect("Error inserting new grado");

    HttpResponse::Ok().json(new_grado)
}

pub async fn get_grados(pool: web::Data<Pool>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");
    let results = grados
        .load::<Grado>(&mut conn)
        .expect("Error loading grados");

    HttpResponse::Ok().json(results)
}

pub async fn get_grado_by_id(
    pool: web::Data<Pool>,
    grado_id: web::Path<i32>,
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");
    let result = grados
        .filter(id.eq(grado_id.into_inner()))
        .first::<Grado>(&mut conn)
        .optional()
        .expect("Error loading grado");

    if let Some(grado) = result {
        HttpResponse::Ok().json(grado)
    } else {
        HttpResponse::NotFound().body("Grado not found")
    }
}

pub async fn update_grado(
    pool: web::Data<Pool>,
    grado_id: web::Path<i32>,
    item: web::Json<GradoInput>,
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    let fecha_obtencion_parsed = NaiveDate::parse_from_str(&item.fecha_obtencion, "%Y-%m-%d")
        .expect("Invalid date format");

    let target = grados.filter(id.eq(grado_id.into_inner()));
    let updated_grado = diesel::update(target)
        .set((
            nombre.eq(&item.nombre),
            fecha_obtencion.eq(fecha_obtencion_parsed),
            obs.eq(item.obs.as_deref()),
            usuario_modificacion.eq(&item.usuario_modificacion),
            fecha_modificacion.eq(chrono::Local::now().naive_local()),
        ))
        .execute(&mut conn)
        .expect("Error updating grado");

    if updated_grado == 0 {
        HttpResponse::NotFound().body("Grado not found")
    } else {
        HttpResponse::Ok().json("Grado updated")
    }
}

pub async fn delete_grado(
    pool: web::Data<Pool>,
    grado_id: web::Path<i32>,
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    let deleted_grado = diesel::delete(grados.filter(id.eq(grado_id.into_inner())))
        .execute(&mut conn)
        .expect("Error deleting grado");

    if deleted_grado == 0 {
        HttpResponse::NotFound().body("Grado not found")
    } else {
        HttpResponse::Ok().json("Grado deleted")
    }
}
