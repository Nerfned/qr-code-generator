use crate::psql;
use axum::{http::HeaderMap, response::Response, Json};
use either::Either;
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct ChangeQR {
    qrid: i64,
    loadqr: Loadqr,
}

use super::{load_session, Loadqr};

pub async fn change_qr(headers: HeaderMap, body: Json<ChangeQR>) -> Response {
    let session = match load_session(&headers).await.unwrap() {
        Either::Left(session) => session,
        Either::Right(error) => return error,
    };

    psql::change_qr(session.user_id, body.qrid, body.loadqr.clone())
        .await
        .unwrap();

    Response::default()
}
