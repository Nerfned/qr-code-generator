use crate::postgresql::get_pool;
use serde::Deserialize;
use sqlx::Row;

#[derive(Deserialize)]
pub struct User {
    pub(crate) id: i64,
    pub(crate) username: String,
    pub(crate) email: String,
    pub(crate) password: String,
}

pub async fn get_by_username(username: &str) -> Result<Option<User>, sqlx::Error> {
    let user =
        sqlx::query("SELECT id, username, email, password FROM \"user\" WHERE username = $1")
            .bind(username)
            .fetch_optional(get_pool())
            .await?;

    let Some(row) = user else { return Ok(None) };

    Ok(Some(User {
        id: row.get("id"),
        username: row.get("username"),
        email: row.get("email"),
        password: row.get("password"),
    }))
}

pub async fn update_email(id: i64, email: &str) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE \"user\" SET email = $1 WHERE id = $2")
        .bind(email)
        .bind(id)
        .execute(get_pool())
        .await?;

    Ok(())
}

pub async fn update_password(id: i64, password: &str) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE \"user\" SET password = $1 WHERE id = $2")
        .bind(password)
        .bind(id)
        .execute(get_pool())
        .await?;

    Ok(())
}
