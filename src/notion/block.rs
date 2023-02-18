use std::str::FromStr;
use std::fmt::Display as FmtDisplay;

use strum::EnumProperty;
use strum_macros::{Display as Enumdisplay, EnumString};
use serde_json::Value;
use super::{Request, NotionModule, CommErr, get_value_str, get_property_value};


#[derive(Enumdisplay, EnumString, EnumProperty, Debug)]
#[strum(serialize_all = "snake_case")] 
pub enum BlockType {
    // rich text
    #[strum(props(md="{}"))]
    Paragraph,
    #[strum(serialize="heading_1")]
    #[strum(props(md="# {}"))]
    Heading1,
    #[strum(serialize="heading_2")]
    #[strum(props(md="## {}"))]
    Heading2,
    #[strum(serialize="heading_3")]
    #[strum(props(md="### {}", cmd="false"))]
    Heading3,
    #[strum(props(md="* {}"))]
    BulletedListItem,
    #[strum(props(md="1. {}"))]
    NumberedListItem,
    #[strum(props(md="- [{status}] {}", cmd="false"))]
    ToDo,
    #[strum(props(md="<details><summary>{}</summary>{child}</details>", cmd="false"))]
    Toggle,
    #[strum(props(md="<aside>{status}{}</aside>", cmd="false"))]
    Callout,
    #[strum(props(md="> {}"))]
    Quote,
    #[strum(props(md="```{status}\n{}\n```"))]
    Code,
    // special
    #[strum(props(md="---"))]
    Divider,
    #[strum(props(md="$${}$$"))]
    Equation,
    Template,
    ChildPage,
    ChildDatabase,
    Embed,
    Image,
    Video,
    File,
    Pdf,
    Bookmark,
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

#[derive(Enumdisplay, EnumString, EnumProperty, Debug)]
#[strum(serialize_all = "snake_case")] 
pub enum Annotation {
    #[strum(props(md="**{}**", mdrpl="__{}__"))]
    Bold,
    #[strum(props(md="*{}*", mdrpl="_{}_"))]
    Italic,
    #[strum(props(md="<del>{}</del>", cmd="false"))]
    Strikethrough,
    #[strum(props(md="<u>{}</u>", cmd="false"))]
    Underline,
    #[strum(props(md="`{}`"))]
    Code,
    #[strum(props(md="<font {color}>{}</font>"))]
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


#[derive(Enumdisplay, EnumString, EnumProperty, Default, Debug)]
#[strum(serialize_all = "snake_case")] 
pub enum AnnoColor {
    #[default] Default,
    #[strum(props(md="color=blue"))]
    Blue,
    #[strum(props(md="style=background:blue"))]
    BlueBackground,
    #[strum(props(md="color=brown"))]
    Brown,
    #[strum(props(md="style=background:brown"))]
    BrownBackground,
    #[strum(props(md="color=gray"))]
    Gray,
    #[strum(props(md="style=background:gray"))]
    GrayBackground,
    #[strum(props(md="color=green"))]
    Green,
    #[strum(props(md="style=background:green"))]
    GreenBackground,
    #[strum(props(md="color=orange"))]
    Orange,
    #[strum(props(md="style=background:orange"))]
    OrangeBackground,
    #[strum(props(md="color=pink"))]
    Pink,
    #[strum(props(md="style=background:pink"))]
    PinkBackground,
    #[strum(props(md="color=purple"))]
    Purple,
    #[strum(props(md="style=background:purple"))]
    PurpleBackground,
    #[strum(props(md="color=red"))]
    Red,
    #[strum(props(md="style=background:red"))]
    RedBackground,
    #[strum(props(md="color=yellow"))]
    Yellow,
    #[strum(props(md="style=background:yellow"))]
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

    fn build_anno(&self) -> String {
        let mut anno_format = "{}".to_string();
        let mut conflict = false;
        for anno in self.annotation.iter() {
            anno_format = match anno {
                Annotation::Color(AnnoColor::Default) => anno_format,
                Annotation::Color(c) => (&anno_format).replace("{}", anno.get_str("md").unwrap()).replace("{color}", c.get_str("md").unwrap()),
                Annotation::Bold|Annotation::Italic => {
                    let anno_prop = if !conflict { conflict = true; "md" } else { "mdrpl" };
                    (&anno_format).replace("{}", anno.get_str(anno_prop).unwrap())
                },
                Annotation::Code => anno.get_str("md").unwrap().to_string(),
                _ => (&anno_format).replace("{}", anno.get_str("md").unwrap()),
            };
        }
        anno_format.replace("{}", &self.text)
    }
}

impl FmtDisplay for RichText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build_anno())
    }
}

#[derive(Debug)]
pub struct Block {
    pub line: Vec<RichText>,
    pub line_type: BlockType,
    pub color: AnnoColor,
    pub child: Vec<Block>,
    pub status: Value,
}

impl Block {
    pub fn new(line_type: BlockType) -> Self {
        Block { line: Vec::new(), line_type, color: AnnoColor::Default, child: Vec::new(), status: Value::default() }
    }

    pub fn from_text(line_type: BlockType, text: String) -> Self {
        Block {
            line: vec![ RichText { text, href: String::default(), annotation: Vec::new() } ],
            line_type,
            color: AnnoColor::default(),
            child: Vec::new(),
            status: Value::default(),
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
            let response = Request::new().get(NotionModule::Blocks, &get_value_str(value, "id")).unwrap();
            for v in response["results"].as_array().unwrap().iter() {
                child.push(Block::from_value(v)?);
            }
        }

        let status = {
            use BlockType::*;
            match line_type {
                Heading1|Heading2|Heading3 => block.get("is_toggleable").unwrap().to_owned(),
                ToDo => block.get("checked").unwrap().to_owned(),
                Callout => block.get("icon").unwrap().to_owned(),
                Code => block.get("language").unwrap().to_owned(),
                _ => Value::default(),
            }
        };

        Ok(Block { line, line_type, color, child, status })
    }
}

impl FmtDisplay for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut paragraph = String::default();
        if self.line.is_empty() {
            paragraph = "<br/>".to_string();
        } else {
            for text in self.line.iter() {
                paragraph += &text.to_string();
            }
        }

        let format = self.line_type.get_str("md").unwrap();
        let status_to_replace = if !self.status.is_null() {
            if self.status.is_boolean() {
                if self.status.as_bool().unwrap() { "x" } else { " " }
            } else if self.status.is_object() {
                get_property_value(&self.status, None).as_str().unwrap()
            } else {
                self.status.as_str().unwrap()
            }
        } else { "" };
        paragraph = format.replace("{}", &paragraph).replace("{status}", status_to_replace);

        paragraph = match self.line_type {
            BlockType::Callout => "\n".to_string() + &paragraph.replace("\n", "<br/>") + "\n",
            BlockType::Quote => "\n".to_string() + &paragraph.replace("\n", "<br/>") + "\n",
            BlockType::Heading1|BlockType::Heading2|BlockType::Heading3|BlockType::Code|BlockType::Toggle|BlockType::Equation|BlockType::Paragraph => "\n".to_string() + &paragraph + "\n",
            _ => paragraph,

        };

        let mut child_paragraph = String::default();
        if !self.child.is_empty() {
            for child in self.child.iter() {
                child_paragraph = if child_paragraph.is_empty() { child_paragraph } else { child_paragraph + "\n"} + &child.to_string();
            }

            paragraph = match self.line_type {
                BlockType::Toggle => paragraph.replace("{child}", &child_paragraph),
                BlockType::Quote => paragraph.trim_end().to_string() + "\n>" + &child_paragraph.trim_start(),
                BlockType::Heading1|BlockType::Heading2|BlockType::Heading3 => paragraph + &child_paragraph,
                _ => paragraph.trim_end().to_string() + "\n\t" + &child_paragraph.replace("\n\n", "\n").replace("\n", "\n\t"),
            };
        }

        paragraph = match self.color {
            AnnoColor::Default => paragraph,
            _ => {
                let color_format = Annotation::Color(AnnoColor::Default).get_str("md").unwrap();
                "\n".to_string() + &color_format.replace("{}", &paragraph).replace("{color}", self.color.get_str("md").unwrap()) + "\n"
            },
        };

        println!("{:#?}", paragraph);
        write!(f, "{}", paragraph)
    }
}