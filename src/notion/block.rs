use std::str::FromStr;

use strum_macros::{Display as EnumDisplay, EnumString};
use serde_json::Value;
use super::{CommErr, get_value_str};


#[derive(EnumDisplay, EnumString, Debug)]
#[strum(serialize_all = "snake_case")] 
pub enum BlockType {
    Paragraph,
    Heading1,
    Heading2,
    Heading3,
    BulletedListItem,
    NumberedListItem,
    ToDo,
    Toggle,
    ChildPage,
    ChildDatabase,
    Embed,
    Image,
    Video,
    File,
    Pdf,
    Bookmark,
    Callout,
    Quote,
    Equation,
    Divider,
    TableOfContents,
    Column,
    ColumnList,
    LinkPreview,
    SyncedBlock,
    Template,
    LinkToPage,
    Table,
    TableRow,
    Unsupported,
    Code,
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

#[derive(Debug)]
pub struct Block {
    pub line: Vec<RichText>,
    pub line_type: BlockType,
    pub color: AnnoColor,
}

impl Block {
    pub fn from_value(value: &Value) -> Result<Self, CommErr> {
        if !value.is_object() {
            return Err(CommErr::CErr("paramter format Wrong!".to_string()));
        }

        let line_type = get_value_str(value, "type");
        let block = value.as_object().unwrap().get(&line_type).unwrap();
        let line_type = match line_type.as_str() {
            "heading_1"|"heading_2"|"heading_3" => line_type.replace("_", ""),
            _ => line_type,
        };
        let line_type = BlockType::from_str(&line_type).unwrap();

        if let BlockType::Divider = line_type {
            return Ok(Block {
                line: Vec::new(),
                line_type: BlockType::Divider,
                color: AnnoColor::Default,
            });
        }

        let rich_text = block.get("rich_text").unwrap().as_array().unwrap();

        let mut line: Vec<RichText> = Vec::new();
        for v in rich_text.iter() {
            let text= v.get(get_value_str(v, "type")).unwrap();
            let href = if text.get("link").unwrap().is_null() {
                String::default()
            } else {
                text.get("link").unwrap().get("url").unwrap().as_str().unwrap().to_string()
            };

            let anno= v.get("annotations").unwrap().as_object().unwrap();
            let mut annotation: Vec<Annotation> = Vec::new();
            for (anno_key, anno_val) in anno.iter() {
                if anno_key == "color" {
                    annotation.push(Annotation::from_str(anno_key).unwrap().reset_val(anno_val.as_str().unwrap()));
                } else if anno_val.as_bool().unwrap() == true {
                    annotation.push(Annotation::from_str(anno_key).unwrap());
                }
            }

            line.push(RichText {
                text: get_value_str(text, "content"),
                href,
                annotation,
            });
        }

        let line_color = get_value_str(block, "color");
        let color  = if !line_color.is_empty() {
            AnnoColor::from_str(&line_color).unwrap()
        } else {
            AnnoColor::default()
        };

        Ok(Block { line, line_type, color })
    }
}
