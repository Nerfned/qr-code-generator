use crate::psql;
use axum::{http::StatusCode, response::Response, Json};
use either::Either;
use serde::Deserialize;

use super::utils;

pub async fn load_email(email: &str) -> Result<Either<Response, Response>, sqlx::Error> {
    match psql::checkemail(email).await {
        false => {
            let mut response = Response::default();
            *response.status_mut() = StatusCode::OK;
            Ok(Either::Left(response))
        }
        true => {
            let mut response = Response::default();
            *response.status_mut() = StatusCode::CONFLICT;

            Ok(Either::Right(response))
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CheckEmail {
    email: String,
}

pub async fn check_email(body: Json<CheckEmail>) -> Response {
    if psql::checkemail(&body.email).await{
        Response::default()
    }else{
        utils::not_found()
    }
}

