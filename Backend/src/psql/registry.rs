
use crate::postgresql::get_pool;

pub async fn registry(username: &str, email: &str, password: &str) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO public.\"user\" (username, email, password) VALUES ($1, $2, $3)")
        .bind(username)
        .bind(email)
        .bind(password)
        .execute(get_pool())
        .await?;

    Ok(())
}
