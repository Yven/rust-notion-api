use std::collections::HashMap;
use std::str::FromStr;
use std::fmt::Display;
use crate::notion::get_property_value;

use super::{filter::Filter, CommErr, get_value_str, Json};
use serde_json::Map;
use strum_macros::{Display as EnumDisplay, EnumString, EnumProperty};
use anyhow::Result;


#[allow(dead_code)]
#[derive(Debug)]
pub struct Author {
    id: String,
    name: String,
    avatar_url: String,
    email: String,
    user_type: String
}

impl Author {
    pub fn new(property_list: &Json) -> Result<Self> {
        let author = get_property_value(property_list, Some("Author"))?;
        Ok(Author {
            id: get_value_str(author, "id")?,
            name: get_value_str(author, "name")?,
            avatar_url: get_value_str(author, "avatar_url")?,
            email: get_value_str(&author["person"], "email")?,
            user_type: get_value_str(author, "type")?,
        })
    }
}


#[derive(EnumDisplay, EnumString, EnumProperty, Debug, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
pub enum PropertyType {
    #[strum(serialize="rich_text",props(update_json=r#""{name}": {"rich_text": [ {"text": {"content": "{value}"} } ] }"#))]
    Text(&'static str),
    #[strum(props(update_json=r#""{name}": {"number": {value}}"#))]
    Number(&'static str),
    #[strum(props(update_json=r#""{name}": {"checkbox": {value}}"#))]
    Checkbox(&'static str),
    #[strum(props(update_json=r#""{name}": {"select": {"name": "{value}"}}"#))]
    Select(&'static str),
    #[strum(props(update_json=r#""{name}": {"multi_select": [ {"name": "{value}"} ] }"#))]
    MultiSelect(&'static str),
    #[strum(props(update_json=r#""{name}": {"status": {"name": "{value}"}}"#))]
    Status(&'static str),
    #[strum(props(update_json=r#""{name}": {"date": {"start": "{value}"}}"#))]
    Date(&'static str),
    People(&'static str),
    Files(&'static str),
    Relation(&'static str),
    Rollup(&'static str),
    Formula(&'static str),
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
        let val: &'static str = Box::leak(Box::new(val));
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
    pub property: PropertyType,
    pub data: Vec<HashMap<String, Json>>,
}

impl Property {
    pub fn new(key: &String, value: &Json) -> Result<Self> {
        let data = get_property_value(value, None)?;
        let type_name = get_value_str(value, "type")?;

        let data = if !data.is_array() {
            vec![data.to_owned()]
        } else {
            data.as_array().ok_or(CommErr::FormatErr("property value"))?.to_owned()
        };

        let mut property_data_opt = Vec::new();
        for arr_val in data.into_iter() {
            let property_map = if arr_val.is_null() {
                Map::new()
            } else if !arr_val.is_object() {
                let mut mp = Map::new();
                mp.insert(type_name.clone(), arr_val);
                mp
            } else {
                arr_val.as_object().ok_or(CommErr::FormatErr("property value"))?.to_owned()
            };

            if !property_map.is_empty() {
                let mut hm = HashMap::new();
                for (k, v) in property_map {
                    hm.insert(k.to_string(), v);
                }
                property_data_opt.push(hm);
            }
        }

        let property = PropertyType::from_str(&type_name).unwrap().reset_val(key.to_string());

        Ok(Property {
            property,
            data: property_data_opt,
        })
    }

    fn data_key(&self) -> String {
        use super::property::PropertyType::*;
        match self.property {
            Date(_) => "start".to_string(),
            Text(_) => "plain_text".to_string(),
            Checkbox(_) => "checkbox".to_string(),
            Number(_) => "number".to_string(),
            _ => "name".to_string(),
        }
    }

    pub fn to_array(&self) -> Result<Vec<String>> {
        if self.data.len() < 1 {
            let msg: &'static str = Box::leak(Box::new(self.property.get_val()));
            return Err(CommErr::FormatErr(msg).into());
        }

        let key = self.data_key();

        let mut res = Vec::new();
        let msg: &'static str = Box::leak(Box::new(key.to_string()));
        for p_item in self.data.iter() {
            res.push(Property::data_to_string(&self.property, p_item.get(&key).ok_or(CommErr::FormatErr(msg))?));
        }

        Ok(res)
    }

    pub fn is_empty(&self) -> bool {
        if self.data.is_empty() { return true; } else { return false; }
    }

    pub fn data_to_string(ptype: &PropertyType, data: &Json) -> String {
        {
            use PropertyType::*;
            match ptype {
                Number(_) => {
                    if data.is_i64() {
                        format!("{}", data.as_f64().unwrap())
                    } else if data.is_u64() {
                        format!("{}", data.as_u64().unwrap())
                    } else if data.is_f64() {
                        format!("{}", data.as_f64().unwrap())
                    } else {
                        format!("{}", data.to_string())
                    }
                },
                Checkbox(_) => format!("{}", data.as_bool().unwrap_or_default()),
                _ => format!("{}", data.as_str().unwrap_or_default()),
            }
        }
    }
}

impl Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let key = self.data_key();

        if self.data.is_empty() {
            return write!(f, "");
        }

        if self.data.len() > 1 {
            return Err(std::fmt::Error);
        }

        write!(f, "{}", Property::data_to_string(&self.property, self.data[0].get(&key).ok_or(std::fmt::Error)?))
    }
}
