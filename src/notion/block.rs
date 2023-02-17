use std::str::FromStr;

use strum_macros::{Display as EnumDisplay, EnumString};
use serde_json::Value;
use super::{CONFIG_MAP, Request, NotionModule, CommErr, get_value_str, get_property_value};


#[derive(EnumDisplay, EnumString, Debug)]
#[strum(serialize_all = "snake_case")] 
pub enum BlockType {
    // rich text
    Paragraph,
    #[strum(serialize="heading_1")]
    Heading1,
    #[strum(serialize="heading_2")]
    Heading2,
    #[strum(serialize="heading_3")]
    Heading3,
    BulletedListItem,
    NumberedListItem,
    ToDo,
    Toggle,
    Callout,
    Quote,
    Template,
    Code,
    // special
    ChildPage,
    ChildDatabase,
    Embed,
    Image,
    Video,
    File,
    Pdf,
    Bookmark,
    Equation,
    Divider,
    TableOfContents,
    Column,
    ColumnList,
    LinkPreview,
    SyncedBlock,
    LinkToPage,
    Table,
    TableRow,
    Unsupported,
}

#[derive(EnumDisplay, EnumString, Debug)]
#[strum(serialize_all = "snake_case")] 
pub enum Annotation {
    Bold,
    Italic,
    Strikethrough,
    Underline,
    Code,
    Color(AnnoColor),
}

impl Annotation {
    pub fn reset_val(self, val: &str) -> Self {
        {
            use Annotation::*;
            match self {
                Color(_) => Color(AnnoColor::from_str(val).unwrap()),
                _ => self,
            }
        }
    }
}


#[derive(EnumDisplay, EnumString, Default, Debug)]
#[strum(serialize_all = "snake_case")] 
pub enum AnnoColor {
    #[default] Default,
    Blue,
    BlueBackground,
    Brown,
    BrownBackground,
    Gray,
    GrayBackground,
    Green,
    GreenBackground,
    Orange,
    RangeBackground,
    Pink,
    PinkBackground,
    Purple,
    PurpleBackground,
    Red,
    RedBackground,
    Yellow,
    YellowBackground,
}

#[derive(Debug)]
pub struct RichText {
    pub text: String,
    pub href: String,
    pub annotation: Vec<Annotation>,
}
impl RichText {
    pub fn new(v: &Value) -> Self {
        let text = get_value_str(v, "plain_text");
        let href = match v.get("href") {
            None => String::default(),
            Some(href) => {
                if href.is_null() {
                    String::default()
                } else {
                    href.as_str().unwrap().to_string()
                }
            }
        };

        let anno= v["annotations"].as_object().unwrap();
        let mut annotation: Vec<Annotation> = Vec::new();
        for (anno_key, anno_val) in anno.iter() {
            if anno_key == "color" {
                annotation.push(Annotation::from_str(anno_key).unwrap().reset_val(anno_val.as_str().unwrap()));
                continue;
            } 
            match anno_val.as_bool() {
                Some(anno_val) => {
                    if anno_val {
                        annotation.push(Annotation::from_str(anno_key).unwrap());
                    }
                },
                _ => continue,
            }
        }

        RichText { text, href, annotation }
    }
}

#[derive(Debug)]
pub struct Block {
    pub line: Vec<RichText>,
    pub line_type: BlockType,
    pub color: AnnoColor,
    pub child: Vec<Block>,
}

impl Block {
    pub fn new(line_type: BlockType) -> Self {
        Block { line: Vec::new(), line_type, color: AnnoColor::Default, child: Vec::new() }
    }

    pub fn from_text(line_type: BlockType, text: String) -> Self {
        Block {
            line: vec![ RichText { text, href: String::default(), annotation: Vec::new() } ],
            line_type,
            color: AnnoColor::default(),
            child: Vec::new(),
        }
    }

    pub fn from_value(value: &Value) -> Result<Self, CommErr> {
        if !value.is_object() {
            return Err(CommErr::CErr("paramter format Wrong!".to_string()));
        }

        let block = get_property_value(value, None);

        let line_type = BlockType::from_str(&get_value_str(value, "type")).unwrap();

        match line_type {
            BlockType::Divider => return Ok(Block::new(line_type)),
            BlockType::Equation => return Ok(Block::from_text(line_type, get_value_str(block, "expression"))),
            _ => (),
        }

        let rich_text = match block.get("rich_text") {
            Some(r) => r.as_array().unwrap(),
            None => return Err(CommErr::CErr("Unsupport Notion Paragraph Format to Reading for now!".to_string())),
        };

        let mut line: Vec<RichText> = Vec::new();
        for v in rich_text.iter() {
            line.push(RichText::new(v));
        }

        let line_color = get_value_str(block, "color");
        let color  = if !line_color.is_empty() {
            AnnoColor::from_str(&line_color).unwrap()
        } else {
            AnnoColor::default()
        };

        let mut child = Vec::new();
        if value.get("has_children").unwrap().as_bool().unwrap() {
            let key = CONFIG_MAP.get("key").unwrap();
            let request = Request::new(key);
            let response = request.get(NotionModule::Blocks, &get_value_str(value, "id")).unwrap();
            let list = response["results"].as_array().unwrap();
            for v in list.iter() {
                child.push(Block::from_value(v)?);
            }
        }

        Ok(Block { line, line_type, color, child })
    }
}
