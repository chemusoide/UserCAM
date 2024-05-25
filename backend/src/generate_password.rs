use bcrypt::{hash, DEFAULT_COST};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: generate_password <password>");
        std::process::exit(1);
    }

    let password = &args[1];
    match hash(password, DEFAULT_COST) {
        Ok(hashed) => println!("Hashed password: {}", hashed),
        Err(err) => eprintln!("Error hashing password: {}", err),
    }
}