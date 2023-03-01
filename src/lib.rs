pub mod notion;
pub mod error;
pub mod db_contents;
use std::env;


use sea_orm::{DatabaseConnection, Database};
use anyhow::Result;

pub async fn db_connection() -> Result<DatabaseConnection> {
    let url = "mysql://".to_string() + 
        &env::var("DB_USER")?
        + ":" + &env::var("DB_PASSWORD")?
        + "@" + &env::var("DB_HOST")?
        + "/" + &env::var("DB_NAME")?;

    Ok(Database::connect(url).await?)
}