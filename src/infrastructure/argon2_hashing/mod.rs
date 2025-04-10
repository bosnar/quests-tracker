use anyhow::{anyhow, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};

pub fn hash(password: String) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow!("Password hashing error: {}", e.to_string()))?
        .to_string();

    Ok(password_hash)
}

pub fn verify(password: String, hash: String) -> Result<bool> {
    let parsed_hash = PasswordHash::new(&hash)
        .map_err(|e| anyhow!("Password hash parsing error: {}", e.to_string()))?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
