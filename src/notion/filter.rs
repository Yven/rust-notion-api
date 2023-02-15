use std::fmt::Display as FmtDisplay;
use std::string::ToString;
use strum_macros::{Display as EnumDisplay, EnumString};


#[derive(EnumDisplay, EnumString, Debug)]
#[strum(serialize_all = "snake_case")] 
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
        {
            use PropertyType::*;
            match self {
                Text(s) |
                Number(s) |
                Checkbox(s) |
                Select(s) |
                MultiSelect(s) |
                Status(s) |
                Date(s) |
                People(s) |
                Files(s) |
                Relation(s) |
                Rollup(s) |
                Formula(s) => s.to_string()
            }
        }
    }

    pub fn reset_val(&self, val: String) -> Self {
        {
            use PropertyType::*;
            match self {
                Text(_) => Text(val),
                Number(_) => Number(val),
                Checkbox(_) => Checkbox(val),
                Select(_) => Select(val),
                MultiSelect(_) => MultiSelect(val),
                Status(_) => Status(val),
                Date(_) => Date(val),
                People(_) => People(val),
                Files(_) => Files(val),
                Relation(_) => Relation(val),
                Rollup(_) => Rollup(val),
                Formula(_) => Formula(val),
            }
        }
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
