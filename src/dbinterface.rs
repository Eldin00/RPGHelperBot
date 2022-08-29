pub mod dbinterface {

    use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
    use std::sync::RwLock;
    use crate::CONF;

    lazy_static! {
       pub static ref DB_POOL: RwLock<Option<SqlitePool>> = RwLock::new(None);
    }

    pub async fn init_db() {
        let db_url = &(*CONF.read().as_deref().unwrap().get_db_url()).to_string();
        if let Ok(db) = DB_POOL.try_write().as_deref_mut() {
            if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
                Sqlite::create_database(db_url).await.unwrap();
            }
            *db = Some(SqlitePool::connect(db_url).await.unwrap());
        }
    }
}
