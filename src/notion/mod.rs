pub mod page;
pub mod database;
pub mod error;
pub mod sort;
pub mod filter;
pub mod property;
pub mod block;
pub mod request;


use std::fmt::Display;

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
pub enum Notion {
    Databases(String),
    Pages(String),
    Blocks(String),
    Users(String),
}

impl Notion {
    pub fn path(&self) -> String {
        match self {
            Notion::Databases(id) => "databases/".to_string() + &id + "/query",
            Notion::Pages(id) => "pages/".to_string() + &id,
            Notion::Blocks(id) => "blocks/".to_string() + &id + "/children",
            Notion::Users(id) => "users/".to_string() + &id,
        }
    }

    pub fn get_val(&self) -> String {
        {
            use Notion::*;
            match self {
                Databases(s) |
                Pages(s) |
                Blocks(s) |
                Users(s) => s.to_string()
            }
        }
    }

    pub fn filter(self, filter: Filter) -> NotionBuilder {
        NotionBuilder { module: self, filter, sort: Sort::default() }
    }

    pub fn sort(self, sort: Vec<(String, Direction)>) -> NotionBuilder  {
        NotionBuilder { module: self, filter: Filter::default(), sort: Sort::new(sort) }
    }
}


pub struct NotionBuilder {
    pub module: Notion,
    filter: Filter,
    sort: Sort,
}

impl NotionBuilder {
    pub fn new(module: Notion) -> Self {
        NotionBuilder { module, filter: Filter::default(), sort: Sort::default() }
    }

    pub fn filter(mut self, filter: Filter) -> Self {
        self.filter = self.filter.and(filter);
        self
    }

    pub fn sort(mut self, order: Vec<(String, Direction)>) -> Self {
        self.sort.add(order);
        self
    }

    // pub fn find(&self) -> T {
    // }

    pub fn search<T: ImpRequest>(&self) -> Result<T> {
        T::search(self.module.get_val(), self.format_body())
    }

    pub fn format_body(&self) -> Json {
        serde_json::from_str::<Json>(&self.to_string()).unwrap()
    }
}

impl Display for NotionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"{{"filter": {},"sorts": {}}}"#, self.filter, self.sort)
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