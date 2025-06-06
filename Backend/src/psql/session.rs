
use serde::Serialize;
use crate::postgresql::get_pool;

pub async fn session(session_id: &str) -> Result<Option<Session>, sqlx::Error> {
    let user_id = sqlx::query_scalar::<_, i64>(
        "SELECT user_id FROM public.\"session\" WHERE session_id = $1",
    )
    .bind(session_id)
    .fetch_optional(get_pool())
    .await?;

    let user_id = match user_id {
        Some(user_id) => user_id,
        None => return Ok(None),
    };

    let username: String = sqlx::query_scalar("SELECT username FROM public.\"user\" WHERE id = $1")
        .bind(user_id)
        .fetch_one(get_pool())
        .await?;

    let email: String = sqlx::query_scalar("SELECT email FROM public.\"user\" WHERE id = $1")
        .bind(user_id)
        .fetch_one(get_pool())
        .await?;

    let icon = sqlx::query_scalar("SELECT icon FROM public.\"usericons\" WHERE userid = $1")
        .bind(user_id)
        .fetch_one(get_pool())
        .await
        .unwrap();

    Ok(Some(Session {
        user_id,
        username,
        email,
        icon,
    }))
}

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Session {
    pub user_id: i64,
    pub username: String,
    pub email: String,
    pub icon: String,
}
