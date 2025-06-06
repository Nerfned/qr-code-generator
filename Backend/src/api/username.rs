use axum::{extract::Json, response::Response};
use serde::Deserialize;

use crate::psql::user;

use super:: utils;

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Username {
    username: String,
}

pub async fn username(body: Json<Username>) -> Response {
    if user::get_by_username(&body.username).await.unwrap().is_some() {
        Response::default()
    }else {
        utils::not_found()
    }
}
