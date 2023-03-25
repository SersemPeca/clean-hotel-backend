use argon2::password_hash::Salt;

pub fn generate_salt() -> String {
    use argon2::password_hash::SaltString;
    use rand::rngs::OsRng;

    SaltString::generate(&mut OsRng).to_string()
}

pub fn hash_password(password: String, salt: String) -> Result<String, String> {
    use argon2::{password_hash::PasswordHasher, Argon2};

    let password = password.as_bytes();

    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(
            password,
            Salt::from_b64(&salt).map_err(|err| format!("Could not parse salt: {}", err))?,
        )
        .map_err(|err| format!("Could not hash password: {}", err))?
        .to_string();

    // NOTE: Argon2 stores the salt in the hash, so
    // we don't need it in the database as a separate field
    Ok(password_hash)
}

pub fn verify_password(password: String, password_hash: String) -> Result<(), String> {
    use argon2::{
        password_hash::{PasswordHash, PasswordVerifier},
        Argon2,
    };

    let password = password.as_bytes();

    let parsed_hash = PasswordHash::new(&password_hash)
        .map_err(|err| format!("Could not hash password: {}", err))?;

    Argon2::default()
        .verify_password(password, &parsed_hash)
        .map_err(|err| format!("Could not verify password: {}", err))
}
