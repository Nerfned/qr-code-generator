use sqlx::{Error, PgPool};

pub async fn create_database_if_not_exists(
    admin_conn_str: &str,
    db_name: &str,
) -> Result<(), Error> {
    println!("{}", db_name);
    dbg!("Before query");
    let admin_pool = match PgPool::connect(admin_conn_str).await {
        Ok(pool) => pool,
        Err(err) => {
            dbg!(err);
            panic!("Config wasnt set properly")
        }
    };

    let exists: Option<i32> = sqlx::query_scalar("SELECT 1 FROM pg_database WHERE datname = $1")
        .bind(db_name)
        .fetch_optional(&admin_pool)
        .await?;

    if exists.is_none() {
        let create_db_query = format!("CREATE DATABASE \"{}\";", db_name);
        sqlx::query(&create_db_query).execute(&admin_pool).await?;
        println!("Database '{}' created.", db_name);
    } else {
        println!("Database '{}' already exists.", db_name);
    }

    Ok(())
}

pub async fn create_tables(pool: &PgPool) -> Result<(), Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS public."user" (
            id BIGSERIAL PRIMARY KEY,
            username VARCHAR(255) NOT NULL UNIQUE,
            email VARCHAR(255) NOT NULL UNIQUE,
            password VARCHAR(255) NOT NULL
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS public."session" (
            session_id VARCHAR(255) PRIMARY KEY,
            session_date DATE NOT NULL,
            user_id BIGINT NOT NULL UNIQUE
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS public."qr" (
            qr_id BIGSERIAL PRIMARY KEY,
            user_id BIGINT NOT NULL,
            qr JSONB NOT NULL
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS public."usericons" (
            usericon_id SERIAL PRIMARY KEY,
            userid BIGINT NOT NULL,
            icon TEXT NOT NULL,
            contenttype VARCHAR(255) NOT NULL
        );
        "#,
    )
    .execute(pool)
    .await?;

    println!("DB Initializ");
    Ok(())
}
