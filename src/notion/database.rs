use super::{page::Page, Json, CommErr, NewImp, get_value_str, Notion};
use anyhow::{Result, Ok};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Database {
    pub id: String,
    pub page_list: Vec<Page>,
    pub next_cursor: String,
    pub has_more: bool,
}

impl NewImp for Database {
    fn new(list: &Json, id: String) -> Result<Self> {
        let result = list.get("results").ok_or(CommErr::FormatErr("results"))?
            .as_array().ok_or(CommErr::FormatErr("results"))?;

        let mut page_list = Vec::new();
        for page in result.iter() {
            page_list.push(Page::new(page, "".to_string())?);
        }

        Ok(Database {
            id,
            page_list,
            next_cursor: get_value_str(list, "next_cursor")?,
            has_more: list.get("has_more").ok_or(CommErr::FormatErr("results"))?.as_bool().unwrap_or_default()
        })
    }

    fn next(&mut self) -> Result<()> {
        if self.has_more {
            self.append(&mut Notion::Databases(self.id.clone()).start_from(self.next_cursor.clone()).search()?);
        }

        Ok(())
    }
}

impl Database {
    pub fn append(&mut self, other: &mut Self) {
        self.page_list.append(&mut other.page_list);
        self.next_cursor = other.next_cursor.clone();
        self.has_more = other.has_more;
    }
}