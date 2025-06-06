use super::get_by_username;
use crate::postgresql::get_pool;

pub async fn save_session(
    sessionid: &str,
    username: &str,
    sessiondate: &str
) -> Result<(), sqlx::Error> {
    let userid = get_by_username(username).await.unwrap().unwrap().id;

    let check_session_exists = sqlx::query("SELECT * FROM public.\"session\" WHERE user_id= $1")
        .bind(userid)
        .fetch_one(get_pool())
        .await
        .is_ok();

    if !check_session_exists {
        sqlx::query("INSERT INTO public.\"session\" (session_id, session_date, user_id) VALUES ($1, $2::date, $3)")
    .bind(sessionid)
    .bind(sessiondate)
    .bind(userid)
    .execute(get_pool())
    .await?;
    } else {
        sqlx::query(
            "UPDATE public.\"session\" SET session_id=$1, session_date=$2::date WHERE user_id=$3",
        )
        .bind(sessionid)
        .bind(sessiondate)
        .bind(userid)
        .execute(get_pool())
        .await?;
    }
    Ok(())
}