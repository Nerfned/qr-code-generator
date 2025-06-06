use super::{load_email::load_email, load_session};
use crate::psql::user;
use axum::{extract::Json, http::HeaderMap, response::Response};
use either::Either;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ChangeEmail {
    new_email: String,
}

pub async fn change_email(headers: HeaderMap, body: Json<ChangeEmail>) -> Response {
    let session = match load_session(&headers).await.unwrap() {
        Either::Left(session) => session,
        Either::Right(error) => return error,
    };

    match load_email(&body.new_email).await.unwrap() {
        Either::Left(response) => response,
        Either::Right(error) => error,
    };

    user::update_email(session.user_id, &body.new_email)
        .await
        .unwrap();

    Response::default()
}
