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
            let key: &'static str = Box::leak(Box::new(k.to_string()));
            let val: &'static str = Box::leak(Box::new(v.as_str().unwrap().to_string()));
            m.insert(key, val);
        }

        m
    };
}