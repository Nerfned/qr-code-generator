use super::{hash_password, load_session, verify_password};
use crate::psql::user;
use axum::{extract::Json, http::HeaderMap, response::Response};
use either::Either;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ChangePassword {
    new_password: String,
    old_password: String,
}

pub async fn change_password(headers: HeaderMap, body: Json<ChangePassword>) -> Response {
    let session = match load_session(&headers).await.unwrap() {
        Either::Left(session) => session,
        Either::Right(error) => return error,
    };

    let hash = user::get_by_username(&session.username)
        .await
        .unwrap()
        .unwrap()
        .password;

    match verify_password(&body.old_password, &hash).unwrap() {
        Either::Left(response) => response,
        Either::Right(error) => return error,
    };

    user::update_password(session.user_id, &hash_password(&body.new_password))
        .await
        .unwrap();

    Response::default()
}
