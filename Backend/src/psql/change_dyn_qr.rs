use crate::api::CreateDynQR;
use crate::postgresql::get_pool;

pub async fn change_dyn_qr(user_id: i64, body: CreateDynQR,) -> Result<(), sqlx::Error> {
        sqlx::query(&format!(
            "UPDATE public.\"dynamic_qrcodes\" SET instagram=$1, facebook=$2, tiktok=$3, youtube=$4, github=$5, linkedin=$6 WHERE user_id={user_id}"
        ))
        .bind(body.instagram)
        .bind(body.facebook)
        .bind(body.tiktok)
        .bind(body.youtube)
        .bind(body.github)
        .bind(body.linkedin)
        .execute(get_pool())
        .await?;
    
    Ok(())
}

