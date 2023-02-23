use super::{request::Request, request::RequestMethod, Notion, get_property_value, get_value_str, property::Property, block::Block, Json, ImpRequest, error::CommErr};
use anyhow::Result;


// 作者信息
#[allow(dead_code)]
#[derive(Debug)]
pub struct Author {
    id: String,
    name: String,
    avatar_url: String,
    email: String,
    user_type: String
}

impl Author {
    pub fn new(property_list: &Json) -> Result<Self> {
        let author = get_property_value(property_list, Some("Author"))?;
        Ok(Author {
            id: get_value_str(author, "id")?,
            name: get_value_str(author, "name")?,
            avatar_url: get_value_str(author, "avatar_url")?,
            email: get_value_str(&author["person"], "email")?,
            user_type: get_value_str(author, "type")?,
        })
    }
}


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

impl Page {
    pub fn new(page: &Json) -> Result<Self> {
        let property_list = page.get("properties").ok_or(CommErr::FormatErr("properties"))?;

        let author = Author::new(property_list)?;

        let mut properties: Vec<Property> = Vec::new();
        for (key, value) in property_list.as_object().ok_or(CommErr::FormatErr("properties"))?.iter() {
            match key.as_str() {
                "Author" | "Created time" | "Edited time" | "Name" => (),
                _ => properties.push(Property::new(key, value)?),
            }
        }

        Ok(Page {
            id: get_value_str(page, "id")?,
            created_time: get_value_str(page, "created_time")?,
            edited_time: get_value_str(page, "last_edited_time")?,
            author,
            editor_id: get_value_str(&page["last_edited_by"], "id")?,
            cover: get_value_str(page, "cover")?,
            icon: get_value_str(page, "icon")?,
            title: get_value_str(
                get_property_value(property_list, Some("Name"))?
                .get(0).ok_or(CommErr::FormatErr("properties"))?
            , "plain_text")?,
            archived: page.get("archived")
                .ok_or(CommErr::FormatErr("archived"))?
                .as_bool().ok_or(CommErr::FormatErr("archived"))?,
            url: get_value_str(page, "url")?,
            properties,
            content: Block::default(),
        })
    }

//     pub fn from_remote(key: String, id: String) -> Self {
//     }

    pub fn content(&mut self) -> Result<String> {
        let response = Request::new(Notion::Blocks(self.id.to_string()).path())?.request(RequestMethod::GET, Json::default())?;
        self.content = Block::new(response.get("results").ok_or(CommErr::FormatErr("results"))?)?;

        Ok(self.content.to_string())
    }
}

impl ImpRequest for Page {
    fn search(module: &Notion, body: Json) -> Result<Self> {
        let page = Request::new(module.path())?.request(RequestMethod::GET, body)?;
        Page::new(&page)
    }
}
