// @generated automatically by Diesel CLI.

diesel::table! {
    alumnos (id) {
        id -> Integer,
        usuario_id -> Integer,
        dojo_id -> Integer,
        fecha_alta -> Date,
        fecha_baja -> Nullable<Date>,
        dojo_cho -> Bool,
        obs -> Nullable<Text>,
        fecha_creacion -> Datetime,
        fecha_modificacion -> Datetime,
        usuario_creacion -> Varchar,
        usuario_modificacion -> Varchar,
    }
}

diesel::table! {
    dojos (id) {
        id -> Integer,
        nombre -> Varchar,
        direccion -> Nullable<Text>,
        telefono -> Nullable<Varchar>,
        obs -> Nullable<Text>,
        fecha_creacion -> Datetime,
        fecha_modificacion -> Datetime,
        usuario_creacion -> Varchar,
        usuario_modificacion -> Varchar,
    }
}

diesel::table! {
    grados (id) {
        id -> Integer,
        nombre -> Varchar,
        fecha_obtencion -> Date,
        obs -> Nullable<Text>,
        fecha_creacion -> Datetime,
        fecha_modificacion -> Datetime,
        usuario_creacion -> Varchar,
        usuario_modificacion -> Varchar,
    }
}

diesel::table! {
    roles (id) {
        id -> Integer,
        nombre -> Varchar,
        obs -> Nullable<Text>,
        fecha_creacion -> Datetime,
        fecha_modificacion -> Datetime,
        usuario_creacion -> Varchar,
        usuario_modificacion -> Varchar,
    }
}

diesel::table! {
    usuarios (id) {
        id -> Integer,
        nombre -> Varchar,
        apellido1 -> Varchar,
        apellido2 -> Nullable<Varchar>,
        email -> Varchar,
        telefono -> Nullable<Varchar>,
        fechanacimiento -> Date,
        dni -> Varchar,
        password -> Varchar,
        imagen_url -> Nullable<Varchar>,
        rol_id -> Integer,
        obs -> Nullable<Text>,
        fecha_creacion -> Datetime,
        fecha_modificacion -> Datetime,
        usuario_creacion -> Varchar,
        usuario_modificacion -> Varchar,
    }
}

diesel::joinable!(alumnos -> dojos (dojo_id));
diesel::joinable!(alumnos -> usuarios (usuario_id));
diesel::joinable!(usuarios -> roles (rol_id));

diesel::allow_tables_to_appear_in_same_query!(
    alumnos,
    dojos,
    grados,
    roles,
    usuarios,
);
