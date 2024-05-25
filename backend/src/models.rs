// src/models.rs

use diesel::prelude::*;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::Queryable;
use serde::{Deserialize, Serialize};
use crate::schema::{usuarios, roles, alumnos, dojos, grados};
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{self, ConnectionManager};

pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Usuario {
    pub id: i32,
    pub nombre: String,
    pub apellido1: String,
    pub apellido2: Option<String>,
    pub email: String,
    pub telefono: Option<String>,
    pub fechanacimiento: NaiveDate,
    pub dni: String,
    pub password: String,
    pub imagen_url: Option<String>,
    pub rol_id: i32,
    pub obs: Option<String>,
    pub fecha_creacion: NaiveDateTime,
    pub fecha_modificacion: NaiveDateTime,
    pub usuario_creacion: String,
    pub usuario_modificacion: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = usuarios)]
pub struct NewUsuario<'a> {
    pub nombre: &'a str,
    pub apellido1: &'a str,
    pub apellido2: Option<&'a str>,
    pub email: &'a str,
    pub telefono: Option<&'a str>,
    pub fechanacimiento: NaiveDate,
    pub dni: &'a str,
    pub password: &'a str,
    pub imagen_url: Option<&'a str>,
    pub rol_id: i32,
    pub obs: Option<&'a str>,
    pub usuario_creacion: &'a str,
    pub usuario_modificacion: &'a str,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Rol {
    pub id: i32,
    pub nombre: String,
    pub obs: Option<String>,
    pub fecha_creacion: NaiveDateTime,
    pub fecha_modificacion: NaiveDateTime,
    pub usuario_creacion: String,
    pub usuario_modificacion: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = roles)]
pub struct NewRol<'a> {
    pub nombre: &'a str,
    pub obs: Option<&'a str>,
    pub usuario_creacion: &'a str,
    pub usuario_modificacion: &'a str,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Alumno {
    pub id: i32,
    pub usuario_id: i32,
    pub dojo_id: i32,
    pub fecha_alta: NaiveDate,
    pub fecha_baja: Option<NaiveDate>,
    pub dojo_cho: bool,
    pub obs: Option<String>,
    pub fecha_creacion: NaiveDateTime,
    pub fecha_modificacion: NaiveDateTime,
    pub usuario_creacion: String,
    pub usuario_modificacion: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = alumnos)]
pub struct NewAlumno<'a> {
    pub usuario_id: i32,
    pub dojo_id: i32,
    pub fecha_alta: NaiveDate,
    pub fecha_baja: Option<NaiveDate>,
    pub dojo_cho: bool,
    pub obs: Option<&'a str>,
    pub usuario_creacion: &'a str,
    pub usuario_modificacion: &'a str,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Dojo {
    pub id: i32,
    pub nombre: String,
    pub direccion: Option<String>,
    pub telefono: Option<String>,
    pub obs: Option<String>,
    pub fecha_creacion: NaiveDateTime,
    pub fecha_modificacion: NaiveDateTime,
    pub usuario_creacion: String,
    pub usuario_modificacion: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = dojos)]
pub struct NewDojo<'a> {
    pub nombre: &'a str,
    pub direccion: Option<&'a str>,
    pub telefono: Option<&'a str>,
    pub obs: Option<&'a str>,
    pub usuario_creacion: &'a str,
    pub usuario_modificacion: &'a str,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Grado {
    pub id: i32,
    pub nombre: String,
    pub fecha_obtencion: NaiveDate,
    pub obs: Option<String>,
    pub fecha_creacion: NaiveDateTime,
    pub fecha_modificacion: NaiveDateTime,
    pub usuario_creacion: String,
    pub usuario_modificacion: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = grados)]
pub struct NewGrado<'a> {
    pub nombre: &'a str,
    pub fecha_obtencion: NaiveDate,
    pub obs: Option<&'a str>,
    pub usuario_creacion: &'a str,
    pub usuario_modificacion: &'a str,
}
