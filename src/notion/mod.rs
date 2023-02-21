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
pub use serde_json::Value as Json;
use anyhow::{Result, anyhow};


trait ImpRequest {
    fn search(id: String, val: Json) -> Result<Self>  where Self: Sized;
    // fn find(&self, val: Json) -> Self;
    // fn save(&self, val: Json) -> Self;
    // fn update(&self, val: Json) -> Self;
    // fn delete(&self, val: Json) -> Self;
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

    pub fn search(&self) -> Result<Database> {
        Database::search(self.module.get_name(), self.format_body())
    }

    // pub fn find(&self) -> T {
    // }

    pub fn format_body(&self) -> Json {
        serde_json::from_str::<Json>(&format!(r#"{{"filter": {},"sorts": {}}}"#, self.filter, self.sort)).unwrap()
    }
}


/**
 * 获取Notion属性数组中的属性值
 */
fn get_property_value<'a>(property: &'a Json, index: Option<&str>) -> Result<&'a Json> {
    let property = match index {
        Some(i) => &property.get(i).ok_or(anyhow!(format!("get_property_value() -> index [{}] do not exist", index.unwrap())))?,
        None => property,
    };

    property.get(get_value_str(property, "type")?).ok_or(anyhow!("get_property_value() -> [type] do not exist"))
}

/**
 * 获取Json中的某个值的String形式
 */
fn get_value_str(value: &Json, index: &str) -> Result<String> {
    Ok(
        value.get(index).ok_or(anyhow!(format!("get_value_str() -> Do not exist [{}] in Json Data.", index)))?
            .as_str().ok_or(anyhow!(format!("get_value_str() -> Not a String Data this property [{}]", index)))?
            .to_string()
    )
}