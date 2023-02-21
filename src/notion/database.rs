use super::{request::Request, request::RequestMethod, page::Page, Module, ImpRequest, Json};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Database {
    pub page_list: Vec<Page>
    // TODO: next_cursor/has_more/type/page属性
}

impl Database {
    pub fn new(list: &Vec<Json>) -> Self {
        let mut page_list = Vec::new();
        for page in list.iter() {
            page_list.push(Page::new(page));
        }

        Database { page_list }
    }
}

impl ImpRequest for Database {
    fn search(id: String, body: Json) -> Self {
        let res = Request::new(Module::Databases(id).path()).request(RequestMethod::POST, body).unwrap();
        Database::new(res["results"].as_array().unwrap())
    }
}