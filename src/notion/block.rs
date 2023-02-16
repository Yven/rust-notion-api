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
    fn from_value(value: &Value) -> Result<Self, CommErr> {
        if !value.is_object() {
            return Err(CommErr::CErr("paramter format Wrong!".to_string()));
        }

        let block = value.as_object().unwrap();
        let line_type = get_value_str(value, "type");
        let rich_text = block.get(&line_type).unwrap().get("rich_text").unwrap().as_array().unwrap();

        let mut line: Vec<RichText> = Vec::new();
        for v in rich_text.iter() {
            let inner = v.get(get_value_str(v, "type")).unwrap();
            let anno_obj= v.get("annotations").unwrap().as_object().unwrap();
            let mut annotation: Vec<Annotation> = Vec::new();
            for (ak, av) in anno_obj.iter() {
                if av.as_bool().unwrap() == true {
                    annotation.push(Annotation::from_str(ak).unwrap());
                }
            }
            line.push(RichText {
                text: get_value_str(inner, "content"),
                href: get_value_str(inner, "href"),
                annotation,
            });
        }

        let line_type = BlockType::from_str(&line_type).unwrap();
        let color = AnnoColor::from_str(&get_value_str(value, "color")).unwrap();

        Ok(Block { line, line_type, color })
    }
}
