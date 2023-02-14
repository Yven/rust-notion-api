use std::fmt::Display as FmtDisplay;
use std::string::ToString;
use strum_macros::Display;

#[derive(Display, Debug)]
pub enum PropertyType {
    Text(String),
    Number(String),
    Checkbox(String),
    Select(String),
    MultiSelect(String),
    Status(String),
    Date(String),
    People(String),
    Files(String),
    Relation(String),
    Rollup(String),
    Formula(String),
}

impl PropertyType {
    pub fn equals(self, val: &str) -> Filter {
        Filter::new(self, ("equals".to_string(), val.to_string()))
    }
    pub fn does_not_equal(self, val: &str) -> Filter {
        Filter::new(self, ("does_not_equal".to_string(), val.to_string()))
    }

    pub fn get_name(&self) -> String {
        let s = match self {
            PropertyType::Text(s) => s,
            PropertyType::Number(s) => s,
            PropertyType::Checkbox(s) => s,
            PropertyType::Select(s) => s,
            PropertyType::MultiSelect(s) => s,
            PropertyType::Status(s) => s,
            PropertyType::Date(s) => s,
            PropertyType::People(s) => s,
            PropertyType::Files(s) => s,
            PropertyType::Relation(s) => s,
            PropertyType::Rollup(s) => s,
            PropertyType::Formula(s) => s,
        };

        s.to_string()
    }
}


#[derive(Debug)]
#[allow(dead_code)]
pub struct Filter {
    property: PropertyType,
    condition: (String, String),
    logic_operate: String,
    logic_map: Vec<Filter>,
}

impl Filter {
    pub fn new(property: PropertyType, condition: (String, String)) -> Self {
        Filter { property, condition, logic_operate: "".to_string(), logic_map: vec![] }
    }

    pub fn has_child(&self) -> bool {
        for v in self.logic_map.iter() {
            if v.logic_map.capacity() != 0 {
                return true
            }
        }

        false
    }

    pub fn and(mut self, val: Filter) -> Self {
        if val.has_child() || self.logic_operate == "or".to_string() {
            return self
        }

        self.logic_operate = "and".to_string();
        self.logic_map.push(val);
        self
    }

    pub fn or(mut self, val: Filter) -> Self {
        if val.has_child() || self.logic_operate == "and".to_string() {
            return self
        }

        self.logic_operate = "or".to_string();
        self.logic_map.push(val);
        self
    }

    // TODO
    pub fn build_str(&self) -> String {
        let mut str = String::from("");

        if self.has_child() {
            for child in self.logic_map.iter() {
                str += child.build_str().as_str();
            }
            return format!(r#"{{"{}":[{}]}}"#, self.logic_operate, str);
        }

        format!(r#"{{"property":"{}","{}":{{"{}":"{}"}}}},"#, self.property.get_name(), self.property.to_string(), self.condition.0, self.condition.1)
    }
}

impl FmtDisplay for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = if self.logic_map.capacity() != 0 {
            format!(r#"{{"and": [{}]}}"#, self.build_str())
        } else {
            format!(r#"{}"#, self.build_str())
        };
        write!(f, "{}", output)
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
pub struct ReqBody {
    filter: Filter,
    sorts: SortMap,
}

impl ReqBody {
    pub fn new(filter: Filter, sorts: SortMap) -> Self{
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