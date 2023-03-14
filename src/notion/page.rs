use super::{Notion, get_property_value, get_value_str, property::Property, property::Author, block::Block, Json, CommErr, NewImp};
use anyhow::Result;


// 页结构
#[derive(Debug)]
pub struct Page {
    pub id: String,
    pub created_time: String,
    pub edited_time: String,
    pub author: Author,
    pub editor_id: String,
    pub cover: String,
    pub icon: String,
    pub title: String,
    pub archived: bool,
    pub url: String,
    pub properties: Vec<Property>,
    pub content: Block,
}

impl NewImp for Page {
    fn new(page: &Json, _: String) -> Result<Self> {
        let property_list = page.get("properties").ok_or(CommErr::FormatErr("properties"))?;

        let author = Author::new(property_list)?;

        let mut properties: Vec<Property> = Vec::new();
        for (key, value) in property_list.as_object().ok_or(CommErr::FormatErr("properties"))?.iter() {
            match key.as_str() {
                "Author" | "Created time" | "Edited time" | "Name" => (),
                _ => properties.push(Property::new(key, value)?),
            }
        }

        let publish_time = Page::search_property_static(&properties, "Publish Time").ok_or(CommErr::FormatErr("Publish Time"))?;

        Ok(Page {
            id: get_value_str(page, "id")?,
            created_time: get_value_str(page, "created_time")?,
            edited_time: if !publish_time.is_empty() { publish_time.to_string() } else { get_value_str(page, "last_edited_time")? },
            author,
            editor_id: get_value_str(&page["last_edited_by"], "id").unwrap_or_default(),
            cover: get_value_str(page, "cover").unwrap_or_default(),
            icon: get_value_str(page, "icon").unwrap_or_default(),
            title: get_value_str(
                get_property_value(property_list, Some("Name"))?
                .get(0).ok_or(CommErr::FormatErr("properties"))?
            , "plain_text")?,
            archived: page.get("archived")
                .ok_or(CommErr::FormatErr("archived"))?
                .as_bool().unwrap_or_default(),
            url: get_value_str(page, "url").unwrap_or_default(),
            properties,
            content: Block::default(),
        })
    }
}

impl Page {
    pub fn content(&mut self) -> Result<String> {
        let block = Notion::Blocks(self.id.to_string()).search::<Block>()?;
        self.content = block;

        Ok(self.content.to_string())
    }

    pub fn search_property(&self, key: &str) -> Option<&Property> {
        Page::search_property_static(&self.properties, key)
    }

    pub fn search_property_static<'a>(properties: &'a Vec<Property>, key: &str) -> Option<&'a Property> {
        for p in properties.iter() {
            if p.property.get_val() == key { return Some(p); }
        }

        None
    }
}
