use std::collections::HashMap;
use std::str::FromStr;
use serde_json::Value;
use super::get_value_str;
use super::filter::Filter;
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
pub struct Property {
    property: PropertyType,
    data: Vec<HashMap<String, String>>,
}

impl Property {
    pub fn new(key: &String, value: &Value) -> Self {
        let mut property_data_opt: Vec<HashMap<String, String>> = Vec::new();

        let data = &value[value["type"].as_str().unwrap().to_string()];
        let data = if !data.is_array() {
            vec![data]
        } else {
            let mut arr_v = Vec::new();
            for v in data.as_array().unwrap().iter() {
                arr_v.push(v);
            }

            arr_v
        };

        for arr_val in data.iter() {
            let mut elem: HashMap<String, String> = HashMap::new();
            for (k, v) in arr_val.as_object().unwrap().iter() {
                if v.is_null() {
                    continue;
                }
                elem.insert(k.to_string(), get_value_str(&arr_val, k));
            }
            property_data_opt.push(elem);
        }

        let property = PropertyType::from_str(&get_value_str(value, "type")).unwrap().reset_val(key.to_string());

        Property {
            property,
            data: property_data_opt,
        }
    }
}

