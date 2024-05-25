# Utiliza la imagen oficial de Rust como base
FROM rust:1.78 as builder

# Crea un nuevo directorio para la aplicación
WORKDIR /usr/src/usercam-api

# Copia los archivos del manifiesto del proyecto al contenedor
# Esto incluye Cargo.toml y posiblemente Cargo.lock
COPY Cargo.toml Cargo.lock ./

# Instala las dependencias necesarias para compilar diesel_cli
RUN apt-get update && \
    apt-get install -y default-libmysqlclient-dev && \
    apt-get clean

# Instala diesel_cli
RUN cargo install diesel_cli --no-default-features --features mysql

# Copia todos los archivos de fuente
COPY . .

# Compila las dependencias para aprovechar la caché de Docker
RUN mkdir -p src/ && \
    echo "// dummy file" > src/lib.rs && \
    cargo build --release --verbose && \
    rm src/lib.rs

# Compila el proyecto entero
RUN cargo build --release --verbose

# Compila el binario generate_password
RUN cargo build --release --bin generate_password

# Etapa de la imagen final.
FROM rust:1.78

# Actualiza los paquetes y limpia para minimizar el tamaño de la imagen
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Copia el ejecutable desde el contenedor de compilación al contenedor final
COPY --from=builder /usr/src/usercam-api/target/release/usercam-api /usr/local/bin/usercam-api
COPY --from=builder /usr/src/usercam-api/target/release/generate_password /usr/local/bin/generate_password
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel

# Define el comando que se ejecutará cuando tu contenedor inicie
CMD ["usercam-api"]
