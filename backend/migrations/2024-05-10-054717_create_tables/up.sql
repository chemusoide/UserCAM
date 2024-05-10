-- Your SQL goes here
CREATE TABLE roles (
    id INT AUTO_INCREMENT PRIMARY KEY,
    nombre VARCHAR(255),
    obs TEXT,
    fechaCreacion DATETIME,
    fechaModificacion DATETIME,
    usuarioCreacion VARCHAR(255),
    usuarioModificacion VARCHAR(255)
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
    rolId INT,
    obs TEXT,
    fechaCreacion DATETIME,
    fechaModificacion DATETIME,
    usuarioCreacion VARCHAR(255),
    usuarioModificacion VARCHAR(255),
    FOREIGN KEY (rolId) REFERENCES roles(id)
);

CREATE INDEX idx_usuario_rolId ON usuarios(rolId);
CREATE INDEX idx_usuario_email ON usuarios(email);
CREATE INDEX idx_usuario_dni ON usuarios(dni);

CREATE TABLE dojos (
    id INT AUTO_INCREMENT PRIMARY KEY,
    nombre VARCHAR(255) NOT NULL,
    direccion TEXT,
    telefono VARCHAR(255),
    obs TEXT,
    fechaCreacion DATETIME,
    fechaModificacion DATETIME,
    usuarioCreacion VARCHAR(255),
    usuarioModificacion VARCHAR(255)
);

CREATE INDEX idx_dojos_nombre ON dojos(nombre);

CREATE TABLE alumnos (
    id INT AUTO_INCREMENT PRIMARY KEY,
    usuarioId INT,
    dojoId INT,
    fechaAlta DATE,
    fechaBaja DATE,
    dojoCho BOOLEAN DEFAULT FALSE,
    obs TEXT,
    fechaCreacion DATETIME,
    fechaModificacion DATETIME,
    usuarioCreacion VARCHAR(255),
    usuarioModificacion VARCHAR(255),
    FOREIGN KEY (usuarioId) REFERENCES usuarios(id),
    FOREIGN KEY (dojoId) REFERENCES dojos(id)
);

CREATE INDEX idx_alumnos_usuarioId ON alumnos(usuarioId);
CREATE INDEX idx_alumnos_dojoId ON alumnos(dojoId);
CREATE INDEX idx_alumnos_fechaAlta ON alumnos(fechaAlta);
CREATE INDEX idx_alumnos_fechaBaja ON alumnos(fechaBaja);
CREATE INDEX idx_alumnos_dojoId_fechaAlta ON alumnos(dojoId, fechaAlta);

CREATE TABLE grados (
    id INT AUTO_INCREMENT PRIMARY KEY,
    nombre VARCHAR(255) NOT NULL,
    fechaObtencion DATE,
    obs TEXT,
    fechaCreacion DATETIME,
    fechaModificacion DATETIME,
    usuarioCreacion VARCHAR(255),
    usuarioModificacion VARCHAR(255)
);

CREATE INDEX idx_grados_fechaObtencion ON grados(fechaObtencion);