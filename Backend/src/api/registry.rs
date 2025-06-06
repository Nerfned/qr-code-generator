use crate::psql::{self, user};
use axum::{extract::Json, response::Response};
use either::Either;
use serde::Deserialize;

use super::{hashing_password::hash_password, load_email::load_email, utils};

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Registry {
    icon: String,
    username: String,
    email: String,
    password: String,
}

pub async fn registry(body: Json<Registry>) -> Response {
    let response = match load_email(&body.username).await.unwrap() {
        Either::Left(response) => response,
        Either::Right(error) => return error,
    };

    if user::get_by_username(&body.username)
        .await
        .unwrap()
        .is_some()
    {
        return utils::conflict();
    }

    psql::registry(&body.username, &body.email, &hash_password(&body.password))
        .await
        .unwrap();

    psql::icons(&body.icon, &body.username).await.unwrap();

    response
}
