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
}

impl Display for ReqBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"{{"filter": {},"sorts": {}}}"#, self.filter, self.sorts)
    }
}
