
// 通用错误返回
#[derive(Debug)]
pub enum CommErr {
    ReqErr(reqwest::Error),
    JsonErr(serde_json::Error),
    CErr(String),
}
impl From<reqwest::Error> for CommErr {
    fn from(error: reqwest::Error) -> Self {
        CommErr::ReqErr(error)
    }
}
impl From<serde_json::Error> for CommErr {
    fn from(error: serde_json::Error) -> Self {
        CommErr::JsonErr(error)
    }
}
impl std::fmt::Display for CommErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommErr::CErr(e) => write!(f, "Error In Notion: {}", e),
            _ => self.fmt(f),
        }
    }
}