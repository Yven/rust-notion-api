pub mod notion;

use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONFIG_MAP: HashMap<&'static str, &'static str> = {
        let path = env!("CARGO_MANIFEST_DIR").to_string() + "/secret.json";
        let config = std::fs::read_to_string(path).unwrap();
        let config = serde_json::from_str::<serde_json::Value>(&config).unwrap();
        let key: &'static str = Box::leak(Box::new(config["key"].as_str().unwrap().to_string()));
        let db_id: &'static str = Box::leak(Box::new(config["db_id"].as_str().unwrap().to_string()));

        let mut m = HashMap::new();
        m.insert("key", key);
        m.insert("db_id", db_id);
        m
    };
}