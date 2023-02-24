use super::{request::Request, request::RequestMethod, page::Page, Notion, ImpRequest, Json, error::CommErr};
use anyhow::Result;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Database {
    pub page_list: Vec<Page>
    // TODO: next_cursor/has_more/type/page属性
}

impl Database {
    pub fn new(list: &Vec<Json>) -> Result<Self> {
        let mut page_list = Vec::new();
        for page in list.iter() {
            page_list.push(Page::new(page)?);
        }

        Ok(Database { page_list })
    }
}

impl ImpRequest for Database {
    fn search(request: &Request, module: &Notion, body: Json) -> Result<Self> {
        let res = request.request(RequestMethod::POST, module.path(), body)?;
        Database::new(
            res.get("results")
            .ok_or(CommErr::FormatErr("results"))?
            .as_array().ok_or(CommErr::FormatErr("results"))?
        )
    }
}