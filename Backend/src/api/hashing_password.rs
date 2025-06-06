use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, Params,
};

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::new(
        argon2::Algorithm::Argon2d,
        argon2::Version::V0x10,
        Params::default(),
    );
    argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}
