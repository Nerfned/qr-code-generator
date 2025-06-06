use super::load_session;
use crate::response::Responses;
use axum::{
    extract::Json,
    http::HeaderMap,
    response::{IntoResponse, Response},
};
use either::Either;

pub async fn send_data(headers: HeaderMap) -> Response {
    let session = match load_session(&headers).await.unwrap() {
        Either::Left(session) => session,
        Either::Right(error) => return error,
    };
    Json(Responses::SendData {
        username: session.username,
        email: session.email,
        icon: session.icon,
    })
    .into_response()
}
