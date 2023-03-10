use super::{page::Page, Json, CommErr, NewImp, get_value_str, Notion};
use anyhow::Result;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Database {
    pub page_list: Vec<Page>,
    pub next_cursor: String,
    pub has_more: bool,
}

impl NewImp for Database {
    fn new(list: &Json) -> Result<Self> {
        let result = list.get("results").ok_or(CommErr::FormatErr("results"))?
            .as_array().ok_or(CommErr::FormatErr("results"))?;

        let mut page_list = Vec::new();
        for page in result.iter() {
            page_list.push(Page::new(page)?);
        }

        Ok(Database {
            page_list,
            next_cursor: get_value_str(list, "next_cursor")?,
            has_more: list.get("has_more").ok_or(CommErr::FormatErr("results"))?.as_bool().unwrap_or_default()
        })
    }

    fn next(&self) -> Result<Self> {
        if !self.has_more {
            return Err(CommErr::CErr("Do not have next page").into());
        }

        Notion::Databases(self.next_cursor.clone()).search()
    }
}