pub mod dbinterface {

    use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
    use std::sync::RwLock;

    lazy_static! {
        static ref DB_POOL: RwLock<Option<SqlitePool>> = RwLock::new(None);
    }

    pub async fn init_db(db_url: &str) {
        if let Ok(db) = DB_POOL.try_write().as_deref_mut() {
            if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
                Sqlite::create_database(db_url).await.unwrap();
            }
            *db = Some(SqlitePool::connect(db_url).await.unwrap());
        }
    }
}
