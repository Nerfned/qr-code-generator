use crate::postgresql::get_pool;

pub async fn checkemail(email: &str) -> bool {
    sqlx::query("SELECT * FROM public.\"user\" WHERE email = $1")
        .bind(email)
        .fetch_one(get_pool())
        .await
        .is_ok()
}
