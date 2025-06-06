use crate::{
    psql,
    qr_json::{Layout, QrData},
};
use axum::{http::HeaderMap, response::Response, Json};
use either::Either;

use super::load_session;
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Loadqr {
    pub qrtitle: String,
    pub data: QrData,
    pub size: String,
    pub layout: Layout,
    pub color: String,
    pub logo: Option<String>,
}

pub async fn save_qr(headers: HeaderMap, body: Json<Loadqr>) -> Response {
    let session = match load_session(&headers).await.unwrap() {
        Either::Left(session) => session,
        Either::Right(error) => return error,
    };

    psql::saveqr(session.user_id, body).await.unwrap();

    Response::default()
}
