-- Your SQL goes here
CREATE TABLE roles (
    id INT AUTO_INCREMENT PRIMARY KEY,
    nombre VARCHAR(255),
    obs TEXT,
    fecha_creacion DATETIME DEFAULT CURRENT_TIMESTAMP,
    fecha_modificacion DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    usuario_creacion VARCHAR(255),
    usuario_modificacion VARCHAR(255)
);

CREATE INDEX idx_roles_nombre ON roles(nombre);

CREATE TABLE usuarios (
    id INT AUTO_INCREMENT PRIMARY KEY,
    nombre VARCHAR(255),
    apellido1 VARCHAR(255),
    apellido2 VARCHAR(255),
    email VARCHAR(255),
    telefono VARCHAR(255),
    fechanacimiento DATE,
    dni VARCHAR(255),
    rol_id INT,
    password VARCHAR(255),
    imagen_url VARCHAR(255),
    obs TEXT,
    fecha_creacion DATETIME DEFAULT CURRENT_TIMESTAMP,
    fecha_modificacion DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    usuario_creacion VARCHAR(255),
    usuario_modificacion VARCHAR(255),
    FOREIGN KEY (rol_id) REFERENCES roles(id)
);

CREATE INDEX idx_usuarios_rol_id ON usuarios(rol_id);
CREATE INDEX idx_usuarios_email ON usuarios(email);
CREATE INDEX idx_usuarios_dni ON usuarios(dni);

CREATE TABLE dojos (
    id INT AUTO_INCREMENT PRIMARY KEY,
    nombre VARCHAR(255) NOT NULL,
    direccion TEXT,
    telefono VARCHAR(255),
    obs TEXT,
    fecha_creacion DATETIME DEFAULT CURRENT_TIMESTAMP,
    fecha_modificacion DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    usuario_creacion VARCHAR(255),
    usuario_modificacion VARCHAR(255)
);

CREATE INDEX idx_dojos_nombre ON dojos(nombre);

CREATE TABLE alumnos (
    id INT AUTO_INCREMENT PRIMARY KEY,
    usuario_id INT,
    dojo_id INT,
    fecha_alta DATE,
    fecha_baja DATE,
    dojo_cho BOOLEAN DEFAULT FALSE,
    obs TEXT,
    fecha_creacion DATETIME DEFAULT CURRENT_TIMESTAMP,
    fecha_modificacion DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    usuario_creacion VARCHAR(255),
    usuario_modificacion VARCHAR(255),
    FOREIGN KEY (usuario_id) REFERENCES usuarios(id),
    FOREIGN KEY (dojo_id) REFERENCES dojos(id)
);

CREATE INDEX idx_alumnos_usuario_id ON alumnos(usuario_id);
CREATE INDEX idx_alumnos_dojo_id ON alumnos(dojo_id);
CREATE INDEX idx_alumnos_fecha_alta ON alumnos(fecha_alta);
CREATE INDEX idx_alumnos_fecha_baja ON alumnos(fecha_baja);
CREATE INDEX idx_alumnos_dojo_id_fecha_alta ON alumnos(dojo_id, fecha_alta);

CREATE TABLE grados (
    id INT AUTO_INCREMENT PRIMARY KEY,
    nombre VARCHAR(255) NOT NULL,
    fecha_obtencion DATE,
    obs TEXT,
    fecha_creacion DATETIME DEFAULT CURRENT_TIMESTAMP,
    fecha_modificacion DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    usuario_creacion VARCHAR(255),
    usuario_modificacion VARCHAR(255)
);

CREATE INDEX idx_grados_fecha_obtencion ON grados(fecha_obtencion);
