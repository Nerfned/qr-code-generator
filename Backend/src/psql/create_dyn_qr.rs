use crate::api::CreateDynQR;
use crate::postgresql::get_pool;

pub async fn create_dyn_qr(user_id:i64, body:CreateDynQR) -> Result<(), sqlx::Error> {
    

    sqlx::query(&format!("INSERT INTO public.\"dynamic_qrcodes\" (user_id, instagram, facebook, tiktok, youtube, linkedin, github) VALUES ({user_id}, $1, $2, $3, $4, $5, $6)"))
        .bind(body.instagram)
        .bind(body.facebook)
        .bind(body.tiktok)
        .bind(body.youtube)
        .bind(body.linkedin)
        .bind(body.github)
        .execute(get_pool())
        .await?;

    Ok(())
}