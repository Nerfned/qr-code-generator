use super::utils;
use argon2::Argon2;
use axum::response::Response;
use either::Either;
use password_hash::{PasswordHash, PasswordVerifier};

pub fn verify_password(
    old_password: &str,
    hash: &str,
) -> Result<Either<(), Response>, sqlx::Error> {
    let parsed_hash = PasswordHash::new(hash).unwrap();
    let ver = Argon2::default()
        .verify_password(old_password.as_bytes(), &parsed_hash)
        .is_ok();

    if ver {
        Ok(Either::Left(()))
    } else {
        Ok(Either::Right(utils::unauthorized()))
    }
}
