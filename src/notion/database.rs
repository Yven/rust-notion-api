use super::{page::Page, Json, error::CommErr, NewImp};
use anyhow::Result;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Database {
    pub page_list: Vec<Page>
    // TODO: next_cursor/has_more/type/page属性
}

impl NewImp for Database {
    fn new(list: &Json) -> Result<Self> {
        let list = list.as_array().ok_or(CommErr::FormatErr("results"))?;

        let mut page_list = Vec::new();
        for page in list.iter() {
            page_list.push(Page::new(page)?);
        }

        Ok(Database { page_list })
    }
}