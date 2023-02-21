use thiserror::Error;


// 通用错误返回
#[derive(Error, Debug)]
pub enum CommErr {
     #[error("Remote Request Error: 【{0}】")]
    ReqErr(#[from] reqwest::Error),
     #[error("Serialize Error: 【{0}】")]
    JsonErr(#[from] serde_json::Error),
     #[error("Notion Module Default Error: 【{0}】")]
    CErr(String),
}