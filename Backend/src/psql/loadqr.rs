use crate::api::Loadqr;

use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use crate::postgresql::get_pool;
#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct Loadingqr {
    pub qr_id: i64,
    pub user_id: Option<i64>,
    pub qr: Json<Loadqr>,
}

pub async fn loadqr(user_id: i64) -> Result<Vec<Loadingqr>, sqlx::Error> {
    let mut rows: Vec<Loadingqr> =
        sqlx::query_as::<_, Loadingqr>("SELECT * FROM public.\"qr\" WHERE user_id=$1")
            .bind(user_id)
            .fetch_all(get_pool())
            .await?;

    rows.sort_by(|a, b| b.qr_id.cmp(&a.qr_id));
    Ok(rows)
}
