// src/handlers/dojos.rs

use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::models::{Dojo, NewDojo, Pool};
use crate::schema::dojos::dsl::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DojoInput {
    pub nombre: String,
    pub direccion: Option<String>,
    pub telefono: Option<String>,
    pub obs: Option<String>,
    pub usuario_creacion: String,
    pub usuario_modificacion: String,
}

pub async fn create_dojo(
    pool: web::Data<Pool>,
    item: web::Json<DojoInput>,
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    let new_dojo = NewDojo {
        nombre: &item.nombre,
        direccion: item.direccion.as_deref(),
        telefono: item.telefono.as_deref(),
        obs: item.obs.as_deref(),
        usuario_creacion: &item.usuario_creacion,
        usuario_modificacion: &item.usuario_modificacion,
    };

    diesel::insert_into(dojos)
        .values(&new_dojo)
        .execute(&mut conn)
        .expect("Error inserting new dojo");

    HttpResponse::Ok().json(new_dojo)
}

pub async fn get_dojos(pool: web::Data<Pool>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");
    let results = dojos
        .load::<Dojo>(&mut conn)
        .expect("Error loading dojos");

    HttpResponse::Ok().json(results)
}

pub async fn get_dojo_by_id(
    pool: web::Data<Pool>,
    dojo_id: web::Path<i32>,
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");
    let result = dojos
        .filter(id.eq(dojo_id.into_inner()))
        .first::<Dojo>(&mut conn)
        .optional()
        .expect("Error loading dojo");

    if let Some(dojo) = result {
        HttpResponse::Ok().json(dojo)
    } else {
        HttpResponse::NotFound().body("Dojo not found")
    }
}

pub async fn update_dojo(
    pool: web::Data<Pool>,
    dojo_id: web::Path<i32>,
    item: web::Json<DojoInput>,
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    let target = dojos.filter(id.eq(dojo_id.into_inner()));
    let updated_dojo = diesel::update(target)
        .set((
            nombre.eq(&item.nombre),
            direccion.eq(item.direccion.as_deref()),
            telefono.eq(item.telefono.as_deref()),
            obs.eq(item.obs.as_deref()),
            usuario_modificacion.eq(&item.usuario_modificacion),
            fecha_modificacion.eq(chrono::Local::now().naive_local()),
        ))
        .execute(&mut conn)
        .expect("Error updating dojo");

    if updated_dojo == 0 {
        HttpResponse::NotFound().body("Dojo not found")
    } else {
        HttpResponse::Ok().json("Dojo updated")
    }
}

pub async fn delete_dojo(
    pool: web::Data<Pool>,
    dojo_id: web::Path<i32>,
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    let deleted_dojo = diesel::delete(dojos.filter(id.eq(dojo_id.into_inner())))
        .execute(&mut conn)
        .expect("Error deleting dojo");

    if deleted_dojo == 0 {
        HttpResponse::NotFound().body("Dojo not found")
    } else {
        HttpResponse::Ok().json("Dojo deleted")
    }
}
