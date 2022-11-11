use sqlx::{SqlitePool, Error, migrate::MigrateDatabase};

pub struct DB;

impl DB {
    pub async fn database_check() -> Result<(), Error> {
        if !sqlx::Sqlite::database_exists("USERDATA.db").await? {
            sqlx::Sqlite::create_database("USERDATA.db").await?;

            let db = SqlitePool::connect("USERDATA.db").await?;
            sqlx::query(include_str!("../sql/221024.sql")).execute(&db).await?;
            db.close().await;
        }
        Ok(())
    }
}