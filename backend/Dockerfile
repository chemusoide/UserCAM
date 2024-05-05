# Utiliza la imagen oficial de Rust como base
FROM rust:1.56 as builder

# Crea un nuevo directorio para la aplicación
WORKDIR /usr/src/usercam-api

# Copia los archivos del manifiesto del proyecto al contenedor
# Esto incluye Cargo.toml y posiblemente Cargo.lock
COPY Cargo.toml Cargo.lock ./

# Este truco con un proyecto ficticio permite cachear las dependencias compiladas,
# reduciendo el tiempo de compilaciones subsiguientes si las dependencias no cambian
RUN mkdir src/ && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs && \
    cargo build --release && \
    rm -f target/release/deps/usercam_api*

# Ahora copia el resto de la aplicación y compila de nuevo.
# Esto asegura que solo se recompilen los archivos que cambian.
COPY . .
RUN touch src/main.rs && cargo build --release

# Etapa de la imagen final
FROM debian:buster-slim

# Actualiza los paquetes y limpia para minimizar el tamaño de la imagen
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Copia el ejecutable desde el contenedor de compilación al contenedor final
COPY --from=builder /usr/src/usercam-api/target/release/usercam-api /usr/local/bin/usercam-api

# Define el comando que se ejecutará cuando tu contenedor inicie
CMD ["usercam-api"]
