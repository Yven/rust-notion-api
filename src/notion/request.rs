use super::{CONFIG_MAP, get_value_str, error::CommErr, Json};
use std::time::Duration;
use reqwest::{self, header::{HeaderMap, HeaderValue, CONTENT_TYPE}};
use anyhow::{Result, anyhow};


const REQ_TIME_S: u64 = 10;
const REQ_TIME_NS: u32 = 0;


#[allow(dead_code)]
pub enum RequestMethod {
    GET,
    POST,
    PATCH,
    DELETE,
}

pub struct Request {
    url: String,
    secret_key: String,
    path: String,
}

impl Request {
    pub fn new(path: String) -> Result<Self> {
        let req = Request {
            url: CONFIG_MAP.get("url").ok_or(anyhow!("config <url> do not exist"))?.to_string(),
            secret_key: CONFIG_MAP.get("key").ok_or(anyhow!("config <key> do not exist"))?.to_string(),
            path,
        };
        Ok(req)
    }

    pub fn request(&self, method: RequestMethod, body: Json) -> Result<Json> {
        let client = reqwest::blocking::Client::new();
        let path = self.url.to_owned() + &self.path;
        let client = match method {
            RequestMethod::GET => client.get(path),
            RequestMethod::POST => {
                client.post(path).json(&body)
            },
            _ => client.get(path),
        };

        let res = client.bearer_auth(&self.secret_key)
            .headers(self.get_header())
            .timeout(Duration::new(REQ_TIME_S, REQ_TIME_NS))
            .send()?;

        let is_success = res.status().is_success();
        let res: Json = serde_json::from_str(res.text()?.as_str())?;
        if is_success {
            Ok(res)
        } else {
            Err(CommErr::CErr(get_value_str(&res, "message")?).into())
        }
    }

    fn get_header(&self) -> HeaderMap {
        let mut header = HeaderMap::new();
        header.insert("Notion-Version", CONFIG_MAP.get("version").unwrap_or(&"2022-06-28").parse().expect("Config [Notion Version] wrong format"));
        header.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        header
    }

    // pub fn get(&self) -> Result<Json, CommErr> {
    //     self.request(RequestMethod::GET, Json::default())
    // }

    // fn save(&self, module: NotionModule) {
    // }

    // fn update(&self, module: NotionModule) {
    // }

    // fn delete(&self, module: NotionModule) {
    // }
}
