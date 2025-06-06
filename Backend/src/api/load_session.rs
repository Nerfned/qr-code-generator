use crate::psql::{session, Session};
use axum::{
    http::{header::AUTHORIZATION, HeaderMap, StatusCode},
    response::Response,
};
use either::Either;

pub async fn load_session(headers: &HeaderMap) -> Result<Either<Session, Response>, sqlx::Error> {
    let session_id = match headers.get(AUTHORIZATION) {
        Some(session_id) => session_id.to_str().unwrap(),
        None => {
            let mut response = Response::default();
            *response.status_mut() = StatusCode::UNAUTHORIZED;

            return Ok(Either::Right(response));
        }
    };

    match session(session_id).await.unwrap() {
        Some(session) => Ok(Either::Left(session)),
        None => {
            let mut response = Response::default();
            *response.status_mut() = StatusCode::UNAUTHORIZED;

            Ok(Either::Right(response))
        }
    }
}
