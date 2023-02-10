#[allow(dead_code)]
#[derive(Default)]
pub struct FilterMap {
}

impl FilterMap {
    pub fn new() -> Self {
        FilterMap {  }
    }

    pub fn as_str(&self) -> &str {
        "{}"
    }
}


#[allow(dead_code)]
#[derive(Default)]
pub struct SortMap {
}

impl SortMap {
    pub fn new() -> Self {
        SortMap {  }
    }

    pub fn as_str(&self) -> &str {
        "{}"
    }
}


#[allow(dead_code)]
#[derive(Default)]
pub struct ReqBody {
    filter: FilterMap,
    sorts: SortMap,
}

impl ReqBody {
    pub fn new(filter: FilterMap, sorts: SortMap) -> Self{
        ReqBody {filter, sorts}
    }

    pub fn as_str(&self) -> String {
        // format!( r#"{{"filter": {},"sorts": {}}}"#, self.filter.as_str(), self.sorts.as_str())

        r#"{
            "filter": {
                "and": [{
                    "property": "Status",
                    "status": {
                        "does_not_equal": "conception"
                    }
                },{
                    "property": "Status",
                    "status": {
                        "does_not_equal": "edit"
                    }
                }
                ]
            },
            "sorts": [{
                "property": "Edited time",
                "direction": "descending"
            }]
        }"#.to_string()
    }
}