use super::CONFIG_MAP;
use super::{Module, get_value_str, error::CommErr};
use std::time::Duration;
use reqwest::{self, header::{HeaderMap, HeaderValue, CONTENT_TYPE}};
use serde_json::Value;


const REQ_TIME_S: u64 = 10;
const REQ_TIME_NS: u32 = 0;


#[allow(dead_code)]
enum RequestMethod {
    GET,
    POST,
    PATCH,
    DELETE,
}


#[allow(dead_code)]
pub struct Request {
    url: String,
    secret_key: String,
    path: String,
}

impl Request {
    pub fn new(module: Module) -> Self {
        Request {
            url: CONFIG_MAP.get("url").unwrap().to_string(),
            secret_key: CONFIG_MAP.get("key").unwrap().to_string(),
            path: module.path(),
        }
    }

    fn request(&self, method: RequestMethod, body: Value) -> Result<Value, CommErr> {
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
        let res: Value = serde_json::from_str(res.text()?.as_str())?;
        if is_success {
            Ok(res)
        } else {
            Err(CommErr::CErr(get_value_str(&res, "message")))
        }
    }

    fn get_header(&self) -> HeaderMap {
        let mut header = HeaderMap::new();
        header.insert("Notion-Version", CONFIG_MAP.get("version").unwrap().parse().unwrap());
        header.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        header
    }

    pub fn query(&self, body: Value) -> Result<Value, CommErr> {
        self.request(RequestMethod::POST, body)
    }

    pub fn get(&self) -> Result<Value, CommErr> {
        self.request(RequestMethod::GET, Value::default())
    }

    // fn save(&self, module: NotionModule) {
    // }

    // fn update(&self, module: NotionModule) {
    // }

    // fn delete(&self, module: NotionModule) {
    // }
}

