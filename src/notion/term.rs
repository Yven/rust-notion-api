use std::fmt::Display;
use super::{sort::Sort, filter::Filter};

#[allow(dead_code)]
pub struct ReqBody {
    filter: Filter,
    sorts: Sort,
}

impl ReqBody {
    pub fn new(filter: Filter, sorts: Sort) -> Self{
        ReqBody { filter, sorts }
    }

    pub fn to_string(&self) -> String {
        format!( r#"{{"filter": {},"sorts": {}}}"#, self.filter.to_string(), self.sorts.to_string())
    }
}

impl Display for ReqBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
