use sqlx::PgPool;
use tokio::sync::OnceCell;

static POOL: OnceCell<PgPool> = OnceCell::const_new();

pub async fn init_db(database_url: &str) {
    println!("{}", database_url);
    let pool = PgPool::connect(database_url).await.unwrap();
    POOL.set(pool).unwrap();
}

pub fn get_pool() -> &'static PgPool {
    POOL.get().expect("Database not initialized")
}