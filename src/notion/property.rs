use std::{collections::HashMap};
use std::str::FromStr;
use crate::notion::get_property_value;

use super::{get_value_str, Json};
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
    pub fn contains(self, val: &str) -> Filter {
        Filter::new(self, ("contains".to_string(), val.to_string()))
    }
    pub fn does_not_contain(self, val: &str) -> Filter {
        Filter::new(self, ("does_not_contain".to_string(), val.to_string()))
    }

    pub fn get_val(&self) -> String {
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
    pub fn new(key: &String, value: &Json) -> Self {
        let data = get_property_value(value, None);
        let property_type = get_value_str(value, "type");

        let data = if !data.is_array() {
            vec![data] 
        } else { 
            let mut vm = Vec::new();
            for v in data.as_array().unwrap() {
                vm.push(v)
            }
            vm
        };

        let mut property_data_opt = Vec::new();
        for arr_val in data.iter() {
            let arr_val = if !arr_val.is_object() {
                vec![(get_value_str(value, "type"), *arr_val)]
            } else {
                let mut vm = Vec::new();
                for (k, v) in arr_val.as_object().unwrap().iter() {
                    vm.push((k.to_string(), v))
                }
                vm
            };

            for (k, v) in arr_val.iter() {
                let v = if v.is_null() {
                    String::default()
                } else {
                    v.as_str().unwrap().to_string()
                };
                let mut hm = HashMap::new();
                hm.insert(k.to_string(), v);
                property_data_opt.push(hm);
            }
        }

        let property = PropertyType::from_str(&property_type).unwrap().reset_val(key.to_string());

        Property {
            property,
            data: property_data_opt,
        }
    }
}

