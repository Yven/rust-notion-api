use thiserror::Error;


// 通用错误返回
#[derive(Error, Debug)]
pub enum CommErr {
     #[error("Remote request error: 【{0}】")]
    ReqErr(#[from] reqwest::Error),
     #[error("Remote API return error: 【{0}】.")]
    HttpResErr(&'static str),
     #[error("Serialize error: 【{0}】.")]
    JsonErr(#[from] serde_json::Error),
     #[error("Module default error: 【{0}】.")]
    CErr(&'static str),
     #[error("Notion remote API returns error json. [{0}] do not exist.")]
    FormatErr(&'static str),
     #[error("[{0}] is Not a string data in the Notion property.")]
    GetValueStrErr(&'static str),
     #[error("[Config setting [{0}] do not exist")]
    ConfigErr(#[from] std::env::VarError),
     #[error("Time string trans to int error: 【{0}】")]
    TimeTransErr(#[from] chrono::ParseError),
     #[error("Database run query error: 【{0}】")]
    DbErr(#[from] sea_orm::DbErr),
     #[error("This is the Default error return: 【{0}】")]
    DefaultErr(#[from] anyhow::Error),
     #[error("Unsupport Notion Paragraph Format to Reading for now!")]
    UnsupportErr,
}