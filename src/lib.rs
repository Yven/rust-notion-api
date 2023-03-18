pub mod notion;
pub mod error;
pub mod entity;

use std::env;
use lazy_static::lazy_static;


lazy_static! {
    pub static ref DEBUG_MODE: bool = {
        match env::var("DEBUG") {
            Ok(data) => data == "true",
            Err(_) => false
        }
    };
}


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