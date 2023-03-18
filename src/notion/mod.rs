pub mod page;
pub mod database;
pub mod sort;
pub mod filter;
pub mod property;
pub mod block;
pub mod request;
pub mod text;


use self::request::{Request, RequestMethod};
use sort::{Sort, Direction};
use filter::Filter;
use property::PropertyType;
use strum::EnumProperty;
pub use super::error::CommErr;

use std::fmt::Display;
pub use serde_json::Value as Json;
use anyhow::Result;


pub trait NewImp {
    fn new(val: &Json, id: String) -> Result<Self>  where Self: Sized;
    fn next(&mut self) -> Result<()> {
        Err(CommErr::CErr("Do not have next page").into())
    }
    // fn search(builder: &NotionBuilder) -> Result<Self>  where Self: Sized;
}

pub enum FnMethod {
    Search,
    Update,
    Create,
    Delete,
}


#[allow(dead_code)]
pub enum Notion {
    Databases(String),
    Pages(String),
    Blocks(String),
    Users(String),
}

impl Notion {
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
        NotionBuilder::new(self).filter(filter)
    }

    pub fn sort(self, field: PropertyType, order: Direction) -> NotionBuilder  {
        NotionBuilder::new(self).sort(field, order)
    }

    pub fn limit(self, page_num: i32) -> NotionBuilder {
        NotionBuilder::new(self).limit(page_num)
    }

    pub fn start_from(self, cursor: String) -> NotionBuilder {
        NotionBuilder::new(self).start_from(cursor)
    }

    pub fn search<T: NewImp>(self) -> Result<T> {
        let id = self.get_val();
        let builder = NotionBuilder::new(self);
        let res = builder.request.query(builder.method(FnMethod::Search), builder.path(), builder.format_body())?;
        T::new(&res, id)
    }

    pub fn update<T: NewImp>(self, data: Vec<(PropertyType, &str)>) -> Result<T> {
        let id = self.get_val();
        let builder = NotionBuilder::new(self);
        let res = builder.request.query(builder.method(FnMethod::Update), builder.path(), NotionBuilder::build_update_body(data)?)?;
        T::new(&res, id)
    }
}


const DEFAULT_PAGE_SIZE: i32 = 100;
pub struct NotionBuilder {
    pub module: Notion,
    request: Request,
    filter: Filter,
    sort: Sort,
    start_cursor: String,
    page_size: i32,
}

impl NotionBuilder {
    pub fn new(module: Notion) -> Self {
        let request = match Request::new() {
            Ok(s) => s,
            Err(e) => panic!("{}", e)
        };
        NotionBuilder {
            module,
            request,
            filter: Filter::default(),
            sort: Sort::default(),
            start_cursor: String::default(),
            page_size: DEFAULT_PAGE_SIZE,
        }
    }

    pub fn path(&self) -> String {
        let mut query_param = Vec::new();
        if !self.start_cursor.is_empty() {
            query_param.push(format!("start_cursor={}", self.start_cursor));
        }

        query_param.push(format!("page_size={}", self.page_size));
        let param = format!("?{}", query_param.join("&"));

        {
            use Notion::*;
            match &self.module {
                Databases(id) => format!("{}{}{}", "databases/", &id, "/query"),
                Pages(id) => format!("{}{}", "pages/", &id),
                Blocks(id) => format!("{}{}{}{}", "blocks/", &id, "/children", param),
                Users(id) => format!("{}{}", "users/", &id),
            }
        }
    }

    pub fn method(&self, act: FnMethod) -> RequestMethod {
        {
            use Notion::*;
            use FnMethod::*;
            match self.module {
                Databases(_) => {
                    match act {
                        Search => RequestMethod::POST,
                        Update => RequestMethod::PATCH,
                        _ => RequestMethod::POST,
                    }
                },
                Pages(_) => {
                    match act {
                        Search => RequestMethod::GET,
                        Update => RequestMethod::PATCH,
                        Create => RequestMethod::POST,
                        _ => RequestMethod::GET,
                    }
                },
                Blocks(_) => {
                    match act {
                        Search => RequestMethod::GET,
                        Update => RequestMethod::PATCH,
                        Create => RequestMethod::POST,
                        Delete => RequestMethod::DELETE,
                    }
                },
                Users(_) => RequestMethod::GET,
            }
        }
    }

    pub fn filter(mut self, filter: Filter) -> Self {
        if self.filter.property.get_val().is_empty() {
            self.filter = filter;
        } else {
            self.filter = self.filter.and(filter);
        }

        self
    }

    pub fn sort(mut self, field: PropertyType, order: Direction) -> Self {
        self.sort.add(vec![(field, order)]);
        self
    }

    // pub fn find(&self) -> T {
    // }

    pub fn limit(mut self, size: i32) -> Self {
        self.page_size = size;
        self
    }

    pub fn start_from(mut self, cursor: String) -> Self {
        self.start_cursor = cursor;
        self
    }

    pub fn search<T: NewImp>(&self) -> Result<T> {
        let res = self.request.query(self.method(FnMethod::Search), self.path(), self.format_body())?;
        T::new(&res, self.module.get_val())
    }

    pub fn build_update_body(data: Vec<(PropertyType, &str)>) -> Result<Json> {
        let mut body: Vec<String> = Vec::new();
        for item in data {
            body.push(item.0.get_str("update_json").ok_or(CommErr::UnsupportErr)?
                .replace("{name}", &item.0.get_val())
                .replace("{value}", item.1));
        }

        Ok(serde_json::from_str::<Json>(&format!(r#"{{"properties": {{{}}}}}"#, body.join(",")))?)
    }

    pub fn format_body(&self) -> Json {
        serde_json::from_str::<Json>(&self.to_string()).unwrap()
    }
}

impl Display for NotionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut body: Vec<String> = Vec::new();

        if !self.start_cursor.is_empty() {
            body.push(format!(r#""start_cursor": "{}""#, self.start_cursor));
        }

        let filter = self.filter.to_string();
        if !filter.is_empty() {
            body.push(format!(r#""filter": {}"#, filter));
        }

        let sort = self.sort.to_string();
        if !sort.is_empty() {
            body.push(format!(r#""sorts": {}"#, sort));
        }

        body.push(format!(r#""page_size": {}"#, self.page_size));

        write!(f, r#"{{{}}}"#, body.join(","))
    }
}


/**
 * 获取Notion属性数组中的属性值
 */
fn get_property_value<'a>(property: &'a Json, index: Option<&'static str>) -> Result<&'a Json> {
    let property = match index {
        Some(i) => &property.get(i).ok_or(CommErr::FormatErr(index.unwrap()))?,
        None => property,
    };

    property.get(get_value_str(property, "type")?).ok_or(CommErr::FormatErr("type").into())
}

/**
 * 获取Json中的某个值的String形式，如果不是String则会返回空字符串
 */
fn get_value_str(value: &Json, index: &'static str) -> Result<String> {
    let val = value.get(index).ok_or(CommErr::FormatErr(index))?;
    Ok(if val.is_string() {
        val.as_str().ok_or(CommErr::GetValueStrErr(index))?.to_string()
    } else {
        String::default()
    })
}

/**
 * 获取图片并转换为base64
 */
fn img_to_base64(path: &str) -> Result<String> {
    match path.find("secure.notion-static.com") {
        Some(_) => return Ok(path.to_string()),
        None => {
            use base64::{Engine as _, engine::general_purpose};
            let res = reqwest::blocking::Client::new().get(path).send()?;
            let code = res.status();
            if code.is_success() {
                let ext = std::path::Path::new(path.split("?").collect::<Vec<&str>>()[0]).extension().unwrap().to_str().unwrap().to_string();
                return Ok(format!("data:image/{};base64,{}", ext, general_purpose::STANDARD.encode(res.bytes()?)));
            }
            Err(CommErr::HttpResErr("Request image in Notin Error").into())
        }
    }
}

/**
 * 下载图片
 */
fn download_img(path: &str, save_path: &str) -> Result<String> {
    if std::path::Path::new(save_path.trim_end_matches('/')).try_exists().is_err() {
        return Err(CommErr::CErr("STATIC_PATH").into());
    }

    let res = reqwest::blocking::Client::new().get(path).send()?;
    let code = res.status();
    if code.is_success() {
        let name = std::path::Path::new(path.split("?").collect::<Vec<&str>>()[0]).file_name().unwrap().to_str().unwrap().to_string();
        let path = format!("{}/{}", save_path.trim_end_matches('/'), name);
        std::fs::write(path.clone(), res.bytes()?)?;
        return Ok(path);
    }
    Err(CommErr::HttpResErr("Request image in Notin Error").into())
}