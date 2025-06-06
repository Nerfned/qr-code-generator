use crate::api::Loadqr;
use sqlx::types::Json;
use crate::postgresql::get_pool;

pub async fn change_qr(user_id: i64, qr_id: i64, qr: Loadqr) -> Result<(), sqlx::Error> {
    let json = Json::try_from(qr).unwrap();

    sqlx::query("UPDATE public.\"qr\" SET user_id=$1,qr=$2 WHERE qr_id=$3")
        .bind(user_id)
        .bind(json)
        .bind(qr_id)
        .execute(get_pool())
        .await?;

    Ok(())
}

