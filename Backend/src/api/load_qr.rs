use crate::{psql, qrdata::qrdata, qrgenerator::qr_generator, response::Responses};
use axum::{
    http::HeaderMap,
    response::{IntoResponse, Response},
    Json,
};
use either::Either;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct ResponseQR {
    qrid: i64,
    svg: String,
    loadqr: Loadqr,
}

use super::{load_session, Loadqr};

pub async fn load_qr(headers: HeaderMap) -> Response {
    let session = match load_session(&headers).await.unwrap() {
        Either::Left(session) => session,
        Either::Right(error) => return error,
    };

    let rows = psql::loadqr(session.user_id).await.unwrap();

    let map = rows
        .into_iter()
        .map(|row| {
            (
                row.qr_id,
                ResponseQR {
                    qrid: row.qr_id,
                    svg: qr_generator(
                        qrdata(row.qr.data.clone()).unwrap(),
                        &row.qr.color,
                        row.qr.layout.clone(),
                        row.qr.logo.clone(),
                    ),
                    loadqr: Loadqr {
                        qrtitle: row.qr.qrtitle.clone(),
                        data: row.qr.data.clone(),
                        size: row.qr.size.clone(),
                        layout: row.qr.layout.clone(),
                        color: row.qr.color.clone(),
                        logo: row.qr.logo.clone(),
                    },
                },
            )
        })
        .collect::<HashMap<_, _>>();

    Json(Responses::LoadQr { qrcodes: map }).into_response()
}
