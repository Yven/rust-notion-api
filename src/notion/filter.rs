use std::fmt::Display as FmtDisplay;
use std::string::ToString;
use super::property::PropertyType;


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
        if val.has_child() || self.logic_operate == *"or" {
            return self
        }

        self.logic_operate = "and".to_string();
        self.logic_map.push(val);
        self
    }

    pub fn or(mut self, val: Filter) -> Self {
        if val.has_child() || self.logic_operate == *"and" {
            return self
        }

        self.logic_operate = "or".to_string();
        self.logic_map.push(val);
        self
    }

    pub fn build_str(&self) -> String {
        if self.property.get_val().is_empty() {
            return "".to_string();
        }

        let mut str = format!(r#"{{"property":"{}","{}":{{"{}":"{}"}}}}"#, self.property.get_val(), self.property.to_string().to_lowercase(), self.condition.0, self.condition.1);

        if self.logic_map.capacity() != 0 {
            for child in self.logic_map.iter() {
                str = str + "," + child.build_str().as_str();
            }
            return format!(r#"{{"{}":[{}]}}"#, self.logic_operate, str);
        }

        str
    }
}

impl Default for Filter {
    fn default() -> Self {
        Filter {
            property: PropertyType::Text(""),
            condition: (String::default(), String::default()),
            logic_operate: String::default(),
            logic_map: Vec::new(),
        }
    }
}

impl FmtDisplay for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build_str())
    }
}
