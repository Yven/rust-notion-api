pub mod page;
pub mod database;
pub mod term;
pub mod error;
pub mod sort;
pub mod filter;
pub mod property;
pub mod block;


use super::CONFIG_MAP;

use error::CommErr;
use term::ReqBody;

use reqwest::{self, header::{HeaderMap, HeaderValue, CONTENT_TYPE}};
use std::time::Duration;
use serde_json::Value;


const NOTION_URL: &str = "https://api.notion.com/v1/";
const REQ_TIME_S: u64 = 10;
const REQ_TIME_NS: u32 = 0;


#[allow(dead_code)]
#[derive(Default)]
pub enum NotionModule {
    #[default] Databases,
    Pages,
    Blocks,
    Users,
}

impl NotionModule {
    fn path(&self, id: &str) -> String {
        match self {
            NotionModule::Databases => {
                "databases/".to_string() + id + "/query"
            },
            NotionModule::Pages => {
                "pages/".to_string() + id
            },
            NotionModule::Blocks => {
                "blocks/".to_string() + id + "/children"
            },
            NotionModule::Users => {
                "users/".to_string() + id
            },
        }
    }
}


#[allow(dead_code)]
#[derive(Default)]
pub struct Request {
    secret_key: String,
}

impl Request {
    pub fn new(secret_key: &str) -> Self {
        Request {
            secret_key: secret_key.to_string(), 
        }
    }

    fn get_header(&self) -> HeaderMap {
        let mut header = HeaderMap::new();
        header.insert("Notion-Version", "2022-06-28".parse().unwrap());
        header.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        header
    }

    pub fn query(&self, module: NotionModule, id: &str, body: ReqBody) -> Result<Value, CommErr> {
        if let NotionModule::Databases = module {
        } else {
            return Err(CommErr::CErr("unsupport module for this method".to_string()));
        }

        let body = serde_json::from_str::<serde_json::Value>(&body.to_string()).unwrap();

        let client = reqwest::blocking::Client::new();
        let res = client.post(NOTION_URL.to_string() + &module.path(id))
            .bearer_auth(&self.secret_key)
            .headers(self.get_header())
            .timeout(Duration::new(REQ_TIME_S, REQ_TIME_NS))
            .json(&body)
            .send()?;

        let is_success = res.status().is_success();
        let res: Value = serde_json::from_str(res.text()?.as_str())?;
        if is_success {
            // Ok(Database::new(res["results"].as_array().unwrap()))
            Ok(res)
        } else {
            Err(CommErr::CErr(get_value_str(&res, "message")))
        }
    }

    fn get(&self, module: NotionModule, id: &str) -> Result<Value, CommErr> {
        let client = reqwest::blocking::Client::new();
        let res = client.get(NOTION_URL.to_string() + &module.path(id))
            .bearer_auth(&self.secret_key)
            .headers(self.get_header())
            .timeout(Duration::new(REQ_TIME_S, REQ_TIME_NS))
            .send()?;

        let is_success = res.status().is_success();
        let res: Value = serde_json::from_str(res.text()?.as_str())?;
        if is_success {
            Ok(res)
        } else {
            Err(CommErr::CErr(get_value_str(&res, "message")))
        }
    }

    // fn save(&self, module: NotionModule) {
    // }

    // fn update(&self, module: NotionModule) {
    // }

    // fn delete(&self, module: NotionModule) {
    // }
}


/**
 * 获取Notion属性数组中的属性值
 */
fn get_property_value<'a>(property: &'a Value, index: &str) -> &'a Value {
    let property_type = property[index]["type"].as_str().unwrap().to_string();
    &property[index][property_type]
}

/**
 * 获取Value中的某个值的String形式
 */
fn get_value_str(value: &Value, index: &str) -> String {
    match value[index].as_str() {
        None => "".to_string(),
        Some(s) => s.to_string(),
    }
}