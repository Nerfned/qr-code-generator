use crate::{psql, response::Responses};
use axum::{
    http::HeaderMap,
    response::{IntoResponse, Response},
    Json,
};
use either::Either;

use super::load_session;
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct QrId {
    pub qrid: i64,
}

pub async fn delete_qr(headers: HeaderMap, body: Json<QrId>) -> Response {
    match load_session(&headers).await.unwrap() {
        Either::Left(session) => session,
        Either::Right(error) => return error,
    };

    let message = psql::delete_qr(body.qrid).await.unwrap();

    if message {
        Json(Responses::Failed).into_response()
    } else {
        Response::default()
    }
}
