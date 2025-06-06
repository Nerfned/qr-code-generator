use crate::postgresql::get_pool;

pub async fn delete_qr(qr_id: i64) -> Result<bool, sqlx::Error> {
    let delete = sqlx::query("DELETE FROM public.\"qr\" WHERE (qr_id=$1)")
        .bind(qr_id)
        .fetch_one(get_pool())
        .await
        .is_ok();

    Ok(delete)
}
