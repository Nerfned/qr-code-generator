use crate::api::Loadqr;
use sqlx::types::Json;

use crate::postgresql::get_pool;

pub async fn saveqr(user_id: i64, qr: axum::Json<Loadqr>) -> Result<(), sqlx::Error> {
    let json = Json::try_from(qr.0).unwrap();

    sqlx::query("INSERT INTO public.\"qr\" (user_id, qr) VALUES ($1, $2)")
        .bind(user_id)
        .bind(json)
        .execute(get_pool())
        .await
        .unwrap();

    Ok(())
}
