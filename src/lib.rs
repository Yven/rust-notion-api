pub mod notion;

use std::collections::HashMap;
use lazy_static::lazy_static;
use serde_json::Value as Json;

lazy_static! {
    pub static ref CONFIG_MAP: HashMap<&'static str, &'static str> = {
        let path = env!("CARGO_MANIFEST_DIR").to_string() + "/secret.json";
        let config = std::fs::read_to_string(path).unwrap();
        let config = serde_json::from_str::<Json>(&config).unwrap();

        let mut m = HashMap::new();
        for (k, v) in config.as_object().unwrap().iter() {
            if !v.is_object() {
                let key: &'static str = Box::leak(Box::new(k.to_string()));
                let val: &'static str = Box::leak(Box::new(v.as_str().unwrap().to_string()));
                m.insert(key, val);
            } else {
                for (ik, iv) in v.as_object().unwrap().iter() {
                    let key: &'static str = Box::leak(Box::new(k.to_string() + "." + ik));
                    let val: &'static str = Box::leak(Box::new(iv.as_str().unwrap().to_string()));
                    m.insert(key, val);
                }
            }
        }

        m
    };
}


pub mod db_contents;

use sea_orm::{DatabaseConnection, Database};
use notion::error::CommErr;
use anyhow::Result;

pub async fn db_connection() -> Result<DatabaseConnection> {
    let url = "mysql://".to_string() + 
        CONFIG_MAP.get("db.user").ok_or(CommErr::ConfigErr("db.user"))?
        + ":" + CONFIG_MAP.get("db.password").ok_or(CommErr::ConfigErr("db.password"))?
        + "@" + CONFIG_MAP.get("db.url").ok_or(CommErr::ConfigErr("db.url"))?
        + "/" + CONFIG_MAP.get("db.name").ok_or(CommErr::ConfigErr("db.name"))?;

    Ok(Database::connect(url).await?)
}