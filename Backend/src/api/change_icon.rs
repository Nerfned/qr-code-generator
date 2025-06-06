use super::load_session;
use crate::psql;
use axum::{extract::Json, http::HeaderMap, response::Response};
use either::Either;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ChangeIcon {
    new_icon: String,
}

pub async fn change_icons(headers: HeaderMap, body: Json<ChangeIcon>) -> Response {
    let session = match load_session(&headers).await.unwrap() {
        Either::Left(session) => session,
        Either::Right(error) => return error,
    };

    psql::change_icon(session.user_id, &body.new_icon)
        .await
        .unwrap();

    Response::default()
}
