use super::user::get_by_username;
use crate::api;
use crate::postgresql::get_pool;

pub async fn icons(icon: &str, username: &str) -> Result<(), sqlx::Error> {
    let user_id = get_by_username(username).await.unwrap().unwrap().id;

    if icon.is_empty() {
        sqlx::query(
            &format!("INSERT INTO public.\"usericons\" (userid, icon, contenttype) VALUES ({user_id}, $1, 'image/svg+xml')"),
        )
        .bind(api::defaultusericon())
        .execute(get_pool())
        .await?;
    } else {
        sqlx::query(
            &format!(
                "INSERT INTO public.\"usericons\" (userid, icon, contenttype) VALUES ({user_id}, $1, 'image/svg+xml')",
            )
        )
        .bind(icon)
        .execute(get_pool())
        .await?;
    }
    Ok(())
}

pub async fn change_icon(id: i64, icon: &str) -> Result<(), sqlx::Error> {
    sqlx::query(&format!(
        "UPDATE  public.\"usericons\" SET icon=$1 WHERE  userid= {id}"
    ))
    .bind(icon)
    .execute(get_pool())
    .await?;
    Ok(())
}

pub async fn get_icon(id: i64) -> Result<String, sqlx::Error> {
    let icon: String = sqlx::query_scalar(&format!(
        "SELECT icon FROM public.\"usericons\" WHERE userid = {id}"
    ))
    .fetch_one(get_pool())
    .await?;
    Ok(icon)
}
//UPDATE public."user" SET password = $1 WHERE username = $2
