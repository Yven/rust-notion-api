use std::{fmt::Display as FmtDisplay, collections::HashMap};
use strum_macros::Display as EnumDisplay;

use super::property::PropertyType;


#[derive(EnumDisplay, Debug)]
pub enum Direction {
    Descending,
    Ascending,
}

pub struct Sort {
    map: HashMap<PropertyType, Direction>
}

impl Sort {
    pub fn new(map: Vec<(PropertyType, Direction)>) -> Self {
        Sort { map: map.into_iter().collect() }
    }

    pub fn add(&mut self, map: Vec<(PropertyType, Direction)>) -> &mut Self {
        self.map.extend(map.into_iter().collect::<HashMap<PropertyType, Direction>>());
        self
    }
}

impl Default for Sort {
    fn default() -> Self {
        Sort { map: HashMap::new() }
    }
}

impl FmtDisplay for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.map.is_empty() {
            return write!(f, "")
        }

        let mut output = String::from("");
        for (k, v) in self.map.iter() {
            output = output + format!(r#"{{"property":"{}","direction":"{}"}}"#, k.get_val(), v.to_string().to_lowercase()).as_str() + ",";
        }
        output.pop();

        write!(f, r#"[{}]"#, output)
    }
}
