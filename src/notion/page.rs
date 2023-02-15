use super::{get_property_value, get_value_str, filter::PropertyType};
use std::collections::HashMap;
use serde_json::Value;
use std::str::FromStr;


// 作者信息
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
    pub fn new(property_list: &Value) -> Self {
        let author = get_property_value(property_list, "Author");
        Author {
            id: get_value_str(author, "id"),
            name: get_value_str(author, "name"),
            avatar_url: get_value_str(author, "avatar_url"),
            email: get_value_str(&author["person"], "email"),
            user_type: get_value_str(author, "type"),
        }
    }
}


// 页属性结构
#[allow(dead_code)]
#[derive(Debug)]
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

        for arr_val in (&data).iter() {
            let mut elem: HashMap<String, String> = HashMap::new();
            for (k, v) in arr_val.as_object().unwrap().iter() {
                if v.is_null() {
                    continue;
                }
                elem.insert(k.to_string(), get_value_str(&arr_val, k));
            }
            property_data_opt.push(elem);
        }

        let enum_name = get_value_str(value, "type");
        let property = PropertyType::from_str(&enum_name).unwrap().reset_val(key.to_string());

        Property {
            property,
            data: property_data_opt,
        }
    }
}


// 页结构
#[derive(Debug)]
pub struct Page {
    pub id: String,
    pub created_time: String,
    pub edited_time: String,
    pub author: Author,
    pub editor_id: String,
    pub cover: String,
    pub icon: String,
    pub title: String,
    pub archived: bool,
    pub url: String,
    pub properties: Vec<Property>,
}

impl Page {
    pub fn new(page: &Value) -> Self {
        let property_list = &page["properties"];

        let author = Author::new(property_list);

        let mut properties: Vec<Property> = Vec::new();
        for (key, value) in property_list.as_object().unwrap().iter() {
            match key.as_str() {
                "Author" | "Created time" | "Edited time" | "Name" => (),
                _ => properties.push(Property::new(key, value)),
            }
        }

        Page {
            id: get_value_str(page, "id"),
            created_time: get_value_str(page, "created_time"),
            edited_time: get_value_str(page, "last_edited_time"),
            author,
            editor_id: get_value_str(&page["last_edited_by"], "id"),
            cover: get_value_str(page, "cover"),
            icon: get_value_str(page, "icon"),
            title: get_value_str(get_property_value(property_list, "Name").get(0).unwrap(), "plain_text"),
            archived: page["archived"].as_bool().unwrap(),
            url: get_value_str(page, "url"),
            properties,
        }
    }
}
