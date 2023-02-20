pub mod page;
pub mod database;
pub mod error;
pub mod sort;
pub mod filter;
pub mod property;
pub mod block;
pub mod request;


use self::{database::Database, sort::Direction};

use super::CONFIG_MAP;
use filter::Filter;
use sort::Sort;

use error::CommErr;
use serde_json::Value;


trait Model {
    fn from_remote(val: Value) -> Self;
}


#[allow(dead_code)]
pub enum Module {
    Databases(String),
    Pages(String),
    Blocks(String),
    Users(String),
}

impl Module {
    pub fn path(&self) -> String {
        match self {
            Module::Databases(id) => "databases/".to_string() + &id + "/query",
            Module::Pages(id) => "pages/".to_string() + &id,
            Module::Blocks(id) => "blocks/".to_string() + &id + "/children",
            Module::Users(id) => "users/".to_string() + &id,
        }
    }

    pub fn get_name(&self) -> String {
        {
            use Module::*;
            match self {
                Databases(s) |
                Pages(s) |
                Blocks(s) |
                Users(s) => s.to_string()
            }
        }
    }

}


pub struct Notion {
    pub module: Module,
    filter: Filter,
    sort: Sort,
}

impl Notion {
    pub fn new(module: Module) -> Self {
        Notion { module, filter: Filter::default(), sort: Sort::default() }
    }

    pub fn filter(mut self, condition: Filter) -> Self {
        self.filter = condition;
        self
    }

    pub fn sort(mut self, order: Vec<(String, Direction)>) -> Self {
        self.sort = Sort::new(order);
        self
    }

    pub fn search(&self) -> Database {
        Database::new(&self.module.get_name(), self.format_body())
    }

    // pub fn find(&self) -> T {
    // }

    pub fn format_body(&self) -> Value {
        serde_json::from_str::<serde_json::Value>(&format!(r#"{{"filter": {},"sorts": {}}}"#, self.filter, self.sort)).unwrap()
    }
}


/**
 * 获取Notion属性数组中的属性值
 */
fn get_property_value<'a>(property: &'a Value, index: Option<&str>) -> &'a Value {
    let property = match index {
        Some(i) => &property[i],
        None => property,
    };

    &property[get_value_str(property, "type")]
}

/**
 * 获取Value中的某个值的String形式
 */
fn get_value_str(value: &Value, index: &str) -> String {
    match value[index].as_str() {
        None => String::default(),
        Some(s) => s.to_string(),
    }
}