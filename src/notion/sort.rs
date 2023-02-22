use std::{fmt::Display as FmtDisplay, collections::HashMap};
use strum_macros::Display as EnumDisplay;

#[derive(EnumDisplay, Debug)]
pub enum Direction {
    Descending,
    Ascending,
}

pub struct Sort {
    map: HashMap<String, Direction>
}

impl Sort {
    pub fn new(map: Vec<(String, Direction)>) -> Self {
        Sort { map: map.into_iter().collect() }
    }

    pub fn add(&mut self, map: Vec<(String, Direction)>) -> &mut Self {
        self.map.extend(map.into_iter().collect::<HashMap<String, Direction>>());
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
        let mut output = String::from("");
        for (k, v) in self.map.iter() {
            output = output + format!(r#"{{"property":"{}","direction":"{}"}}"#, k, v.to_string().to_lowercase()).as_str() + ",";
        }
        output.pop();

        write!(f, r#"[{}]"#, output)
    }
}
