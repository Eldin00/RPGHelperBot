pub mod dbinterface {

    use once_cell::sync::OnceCell;
    use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
    use crate::CONF;

    pub static DB_POOL: OnceCell<SqlitePool> = OnceCell::new();

    pub async fn init_db() {
        let db_url = &(*CONF.read().as_deref().unwrap().get_db_url()).to_string();
        if DB_POOL.get().is_none() {
            if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
                Sqlite::create_database(db_url).await.unwrap();
            }
            _ = DB_POOL.set(SqlitePool::connect(db_url).await.unwrap());
        }
    }
}
