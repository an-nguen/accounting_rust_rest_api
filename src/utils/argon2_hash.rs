use std::env;
use argon2::Config;

fn get_salt() -> String {
    env::var("SALT").unwrap_or_else(|_| String::from("randomsalt"))
}

pub fn hash_password(password: &String) -> String {
    let password_bytes = password.as_bytes();
    let salt = get_salt();
    let salt_bytes = salt.as_bytes();
    let config = Config::default();
    argon2::hash_encoded(password_bytes, salt_bytes, &config).unwrap()
}

pub fn verify_password(password: &String, hash: &String) -> bool {
    argon2::verify_encoded(hash, password.as_bytes()).unwrap()
}