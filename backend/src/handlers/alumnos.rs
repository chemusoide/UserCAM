// src/handlers/alumnos.rs

use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::models::{Alumno, NewAlumno, Pool};
use crate::schema::alumnos::dsl::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use crate::schema::alumnos::{id, usuario_id, dojo_id, dojo_cho, fecha_alta, fecha_baja, obs, usuario_modificacion, fecha_modificacion};

#[derive(Deserialize, Serialize)]
pub struct AlumnoInput {
    pub usuario_id: i32,
    pub dojo_id: i32,
    pub dojo_cho: bool,
    pub fecha_alta: String,
    pub fecha_baja: Option<String>,
    pub obs: Option<String>,
    pub usuario_creacion: String,
    pub usuario_modificacion: String,
}

pub async fn create_alumno(
    pool: web::Data<Pool>,
    item: web::Json<AlumnoInput>,
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    let fecha_alta_parsed = NaiveDate::parse_from_str(&item.fecha_alta, "%Y-%m-%d")
        .expect("Invalid date format");
    let fecha_baja_parsed = item.fecha_baja.as_deref()
        .map(|date_str| NaiveDate::parse_from_str(date_str, "%Y-%m-%d").expect("Invalid date format"));

    let new_alumno = NewAlumno {
        usuario_id: item.usuario_id,
        dojo_id: item.dojo_id,
        dojo_cho: item.dojo_cho,
        fecha_alta: fecha_alta_parsed,
        fecha_baja: fecha_baja_parsed,
        obs: item.obs.as_deref(),
        usuario_creacion: &item.usuario_creacion,
        usuario_modificacion: &item.usuario_modificacion,
    };

    diesel::insert_into(alumnos)
        .values(&new_alumno)
        .execute(&mut conn)
        .expect("Error inserting new alumno");

    HttpResponse::Ok().json(new_alumno)
}

pub async fn get_alumnos(pool: web::Data<Pool>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");
    let results = alumnos
        .load::<Alumno>(&mut conn)
        .expect("Error loading alumnos");

    HttpResponse::Ok().json(results)
}

pub async fn get_alumno_by_id(
    pool: web::Data<Pool>,
    alumno_id: web::Path<i32>,
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");
    let result = alumnos
        .filter(id.eq(alumno_id.into_inner()))
        .first::<Alumno>(&mut conn)
        .optional()
        .expect("Error loading alumno");

    if let Some(alumno) = result {
        HttpResponse::Ok().json(alumno)
    } else {
        HttpResponse::NotFound().body("Alumno not found")
    }
}

pub async fn update_alumno(
    pool: web::Data<Pool>,
    alumno_id: web::Path<i32>,
    item: web::Json<AlumnoInput>,
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    let fecha_alta_parsed = NaiveDate::parse_from_str(&item.fecha_alta, "%Y-%m-%d")
        .expect("Invalid date format");
    let fecha_baja_parsed = item.fecha_baja.as_deref()
        .map(|date_str| NaiveDate::parse_from_str(date_str, "%Y-%m-%d").expect("Invalid date format"));

    let target = alumnos.filter(id.eq(alumno_id.into_inner()));
    let updated_alumno = diesel::update(target)
        .set((
            usuario_id.eq(item.usuario_id),
            dojo_id.eq(item.dojo_id),
            dojo_cho.eq(item.dojo_cho),
            fecha_alta.eq(fecha_alta_parsed),
            fecha_baja.eq(fecha_baja_parsed),
            obs.eq(item.obs.as_deref()),
            usuario_modificacion.eq(&item.usuario_modificacion),
            fecha_modificacion.eq(chrono::Local::now().naive_local()),
        ))
        .execute(&mut conn)
        .expect("Error updating alumno");

    if updated_alumno == 0 {
        HttpResponse::NotFound().body("Alumno not found")
    } else {
        HttpResponse::Ok().json("Alumno updated")
    }
}

pub async fn delete_alumno(
    pool: web::Data<Pool>,
    alumno_id: web::Path<i32>,
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    let deleted_alumno = diesel::delete(alumnos.filter(id.eq(alumno_id.into_inner())))
        .execute(&mut conn)
        .expect("Error deleting alumno");

    if deleted_alumno == 0 {
        HttpResponse::NotFound().body("Alumno not found")
    } else {
        HttpResponse::Ok().json("Alumno deleted")
    }
}
