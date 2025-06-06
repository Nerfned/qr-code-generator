use super::{utils, verify_password};
use crate::{
    psql::{get_icon, save_session, user},
    response::Responses,
};

use axum::{
    extract::Json,
    response::{IntoResponse, Response},
};
use chrono::Utc;
use either::Either;
use randomizer::Randomizer;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Login {
    username: String,
    password: String,
}

pub async fn login(body: Json<Login>) -> Response {
    let user = match user::get_by_username(&body.username).await.unwrap() {
        Some(user) => user,
        None => return utils::not_found(),
    };

    if let Either::Right(error) = verify_password(&body.password, &user.password).unwrap() {
        return error;
    }

    let icon = get_icon(user.id).await.unwrap();
    let session_id = Randomizer::ALPHANUMERIC(64).string().unwrap();

    let sessiondate = Utc::now().format("%Y-%m-%d").to_string();

    save_session(&session_id, &user.username, &sessiondate)
        .await
        .unwrap();

    Json(Responses::SessionSaved {
        id: session_id,
        email: user.email,
        icon,
    })
    .into_response()
}
