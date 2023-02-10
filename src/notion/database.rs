use super::page::Page;
use serde_json::Value;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Database {
    page_list: Vec<Page>
    // TODO: next_cursor/has_more/type/page属性
}

impl Database {
    pub fn new(list: &Vec<Value>) -> Self {
        let mut page_list = Vec::new();
        for page in list.iter() {
            page_list.push(Page::new(page));
        }

        Database { page_list }
    }
}