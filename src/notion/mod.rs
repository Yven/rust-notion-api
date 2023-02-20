pub mod page;
pub mod database;
pub mod term;
pub mod error;
pub mod sort;
pub mod filter;
pub mod property;
pub mod block;
pub mod request;


use super::CONFIG_MAP;
use filter::Filter;
use sort::Sort;

use error::CommErr;
use serde_json::Value;


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
}


pub struct Notion {
    pub module: Module,
    filter: Filter,
    sort: Sort,
}

impl Notion {
    fn new(module: Module) -> Self {
        Notion { module, filter: Filter::default(), sort::default() }
    }

    pub fn filter(self, condition: Filter) -> Self {
        self.filter = condition;
        self
    }

    pub fn sort(self, order: Sort) -> Self {
        self.sort = order;
        self
    }

    // pub fn search(&self) -> T {
    // }

    // pub fn find(&self) -> T {
    // }

    fn format_body(&self) -> Value {
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