use super::{request::Request, page::Page, Module, Model, get_value_str};
use serde_json::Value;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Database {
    pub page_list: Vec<Page>
    // TODO: next_cursor/has_more/type/page属性
}

impl Database {
    pub fn new(id: &str, body: Value) -> Self {
        let response = Request::new(Module::Databases(id.to_string())).query(body).unwrap();
        let mut page_list = Vec::new();
        for page in response["results"].as_array().unwrap().iter() {
            page_list.push(Page::new(page));
        }

        Database { page_list }
    }
}

impl Model for Database {
    fn from_remote(body: Value) -> Self {
        Database::new(&get_value_str(&body, "id"), body)
    }
}