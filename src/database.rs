use std::env;

use sea_orm::*;

const DB_NAME: &str = "neondb";

pub(super) async fn set_up_db() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(env::var("DATABASE_URL").unwrap()).await?;

    let db = match db.get_database_backend() {
        DbBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE IF NOT EXISTS `{}`;", DB_NAME),
            ))
            .await?;

            let url = format!("{}/{}", env::var("DATABASE_URL").unwrap(), DB_NAME);
            Database::connect(&url).await?
        }
        DbBackend::Postgres => {

            let url = format!("{}/{}", env::var("DATABASE_URL").unwrap(), DB_NAME);
            Database::connect(&url).await?
        }
        DbBackend::Sqlite => db,
    };

    Ok(db)
}