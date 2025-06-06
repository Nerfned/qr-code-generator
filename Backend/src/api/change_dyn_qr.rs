use axum::{response::{Response, IntoResponse},http::HeaderMap, Json};
use either::Either;
use super::{hash_password, CreateDynQR};

use crate::{psql, response::Responses};

use super::load_session;

pub async fn change_dyn_qr(headers: HeaderMap, body: Json<CreateDynQR>) -> Response {
    let session = match load_session(&headers).await.unwrap() {
        Either::Left(session) => session,
        Either::Right(error) => return error,
    };

     psql::change_dyn_qr(session.user_id, body.0).await.unwrap();

   Json(Responses::CreateDynQR { url: format!("{}", hash_password(&session.username))}).into_response()
}
