use axum::{response::{Response, IntoResponse},http::HeaderMap, Json};
use either::Either;
use serde::Deserialize;
use super::hash_password;
use crate::{psql, response::Responses};
use super::load_session;

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CreateDynQR {
    pub instagram: String,
    pub facebook: String,
    pub tiktok: String,
    pub youtube: String,
    pub linkedin: String,
    pub github: String
}

pub async fn create_dyn_qr(headers: HeaderMap, body: Json<CreateDynQR>) -> Response {
    let session = match load_session(&headers).await.unwrap() {
        Either::Left(session) => session,
        Either::Right(error) => return error,
    };

     psql::create_dyn_qr(session.user_id, body.0).await.unwrap();

   Json(Responses::CreateDynQR { url: format!("{}", hash_password(&session.username))}).into_response()
}
