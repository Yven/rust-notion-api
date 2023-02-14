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

    pub fn to_string(&self) -> String {
        let mut str = format!(r#"{{"property":"{}","{}":{{"{}":"{}"}}}}"#, self.property.get_name(), self.property.to_string().to_lowercase(), self.condition.0, self.condition.1);

        if self.logic_map.capacity() != 0 {
            for child in self.logic_map.iter() {
                str = str + "," + child.to_string().as_str();
            }
            return format!(r#"{{"{}":[{}]}}"#, self.logic_operate, str);
        }

        str
    }
}

impl FmtDisplay for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}


#[derive(Display, Debug)]
pub enum Direction {
    Descending,
    Ascending,
}

#[allow(dead_code)]
pub struct SortMap {
    property: String,
    direction: Direction,
}

impl SortMap {
    pub fn new(property: String, direction: Direction) -> Self {
        SortMap { property, direction }
    }

    pub fn to_string(&self) -> String {
        format!(r#"{{"property":"{}","direction":"{}"}}"#, self.property, self.direction.to_string().to_lowercase())
    }
}

impl FmtDisplay for SortMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}


#[allow(dead_code)]
pub struct ReqBody {
    filter: Filter,
    sorts: Vec<SortMap>,
}

impl ReqBody {
    pub fn new(filter: Filter, sorts: Vec<SortMap>) -> Self{
        ReqBody { filter, sorts }
    }

    pub fn to_string(&self) -> String {
        let mut sorts = String::from("");
        for s in self.sorts.iter() {
            sorts = sorts + &s.to_string() + ",";
        }
        sorts.pop();

        format!( r#"{{"filter": {},"sorts": [{}]}}"#, self.filter.to_string(), sorts)
    }
}

impl FmtDisplay for ReqBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
