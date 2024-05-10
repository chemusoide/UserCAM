-- This file should undo anything in `up.sql`
-- Remove all indexes first
DROP INDEX IF EXISTS idx_grados_fechaObtencion ON grados;
DROP INDEX IF EXISTS idx_alumnos_dojoId_fechaAlta ON alumnos;
DROP INDEX IF EXISTS idx_alumnos_fechaBaja ON alumnos;
DROP INDEX IF EXISTS idx_alumnos_fechaAlta ON alumnos;
DROP INDEX IF EXISTS idx_alumnos_dojoId ON alumnos;
DROP INDEX IF EXISTS idx_alumnos_usuarioId ON alumnos;
DROP INDEX IF EXISTS idx_dojos_nombre ON dojos;
DROP INDEX IF EXISTS idx_usuario_dni ON usuarios;
DROP INDEX IF EXISTS idx_usuario_email ON usuarios;
DROP INDEX IF EXISTS idx_usuario_rolId ON usuarios;
DROP INDEX IF EXISTS idx_roles_nombre ON roles;

-- Then drop the tables
DROP TABLE IF EXISTS grados;
DROP TABLE IF EXISTS alumnos;
DROP TABLE IF EXISTS dojos;
DROP TABLE IF EXISTS usuarios;
DROP TABLE IF EXISTS roles;
