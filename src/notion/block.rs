use std::str::FromStr;
use std::fmt::Display as FmtDisplay;
use anyhow::Result;

use strum::EnumProperty;
use strum_macros::{Display as Enumdisplay, EnumString};
use super::{request::Request, request::RequestMethod, Notion, CommErr, get_value_str, get_property_value, Json, ImpRequest};
use serde_json::Map;


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
    pub fn new(val: &Json) -> Result<Self> {
        let text = get_value_str(val, "plain_text")?;
        let href = match val.get("href") {
            None => String::default(),
            Some(href) => {
                if href.is_null() {
                    String::default()
                } else {
                    href.as_str().unwrap_or_default().to_string()
                }
            }
        };

        let anno = match val.get("annotations").ok_or(CommErr::FormatErr("annotations"))?.as_object() {
            Some(obj) => obj.to_owned(),
            None => Map::new(),
        };
        let mut annotation: Vec<Annotation> = Vec::new();
        for (anno_key, anno_val) in anno.iter() {
            if anno_key == "color" {
                annotation.push(Annotation::from_str(anno_key)?.reset_val(anno_val.as_str().unwrap()));
                continue;
            } 
            match anno_val.as_bool() {
                Some(anno_val) => {
                    if anno_val {
                        annotation.push(Annotation::from_str(anno_key)?);
                    }
                },
                _ => continue,
            }
        }

        Ok(RichText { text, href, annotation })
    }
}

impl FmtDisplay for RichText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

        write!(f, "{}", anno_format.replace("{}", &self.text))
    }
}

#[derive(Debug)]
pub struct BlockElement {
    pub line: Vec<RichText>,
    pub line_type: BlockType,
    pub color: AnnoColor,
    pub child: Vec<BlockElement>,
    pub status: Json,
}

impl BlockElement {
    pub fn from_type(line_type: BlockType) -> Self {
        BlockElement { line: Vec::new(), line_type, color: AnnoColor::Default, child: Vec::new(), status: Json::default() }
    }

    pub fn from_text(line_type: BlockType, text: String) -> Self {
        BlockElement {
            line: vec![ RichText { text, href: String::default(), annotation: Vec::new() } ],
            line_type,
            color: AnnoColor::default(),
            child: Vec::new(),
            status: Json::default(),
        }
    }

    pub fn new(value: &Json) -> Result<Self> {
        if !value.is_object() {
            return Err(CommErr::FormatErr("results").into());
        }

        let block = get_property_value(value, None)?;

        let line_type = BlockType::from_str(&get_value_str(value, "type")?)?;

        match line_type {
            BlockType::Divider => return Ok(BlockElement::from_type(line_type)),
            BlockType::Equation => return Ok(BlockElement::from_text(line_type, get_value_str(block, "expression")?)),
            _ => (),
        }

        let rich_text = block.get("rich_text")
            .ok_or(CommErr::UnsupportErr)?
            .as_array().ok_or(CommErr::FormatErr("rich text"))?;

        let mut line: Vec<RichText> = Vec::new();
        for v in rich_text.iter() {
            line.push(RichText::new(v)?);
        }

        let line_color = get_value_str(block, "color").unwrap_or_default();
        let color  = if !line_color.is_empty() {
            AnnoColor::from_str(&line_color).unwrap_or_default()
        } else {
            AnnoColor::default()
        };

        // TODO: 异步
        let mut child = Vec::new();
        if value.get("has_children")
            .ok_or(CommErr::FormatErr("has_children"))?
            .as_bool().ok_or(CommErr::FormatErr("has_children"))?
        {
            let response = Request::new(Notion::Blocks(get_value_str(value, "id")?).path())?.request(super::request::RequestMethod::GET, Json::default())?;
            for v in response.get("results")
                .ok_or(CommErr::FormatErr("results"))?
                .as_array().ok_or(CommErr::FormatErr("results"))?.iter()
            {
                child.push(BlockElement::new(v)?);
            }
        }

        let status = {
            use BlockType::*;
            match line_type {
                Heading1|Heading2|Heading3 => block.get("is_toggleable").ok_or(CommErr::FormatErr("is_toggleable"))?.to_owned(),
                ToDo => block.get("checked").ok_or(CommErr::FormatErr("checked"))?.to_owned(),
                Callout => block.get("icon").ok_or(CommErr::FormatErr("icon"))?.to_owned(),
                Code => block.get("language").ok_or(CommErr::FormatErr("language"))?.to_owned(),
                _ => Json::default(),
            }
        };

        Ok(BlockElement { line, line_type, color, child, status })
    }
}

impl FmtDisplay for BlockElement {
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
        let status_to_replace = if self.status.is_null() {
            ""
        } else if self.status.is_boolean() {
            if self.status.as_bool().unwrap() { "x" } else { " " }
        } else if self.status.is_object() {
            match get_property_value(&self.status, None) {
                Ok(v) => v.as_str().unwrap_or_default(),
                Err(_) => "",
            }
        } else {
            self.status.as_str().unwrap()
        };
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

        write!(f, "{}", paragraph)
    }
}

#[derive(Debug)]
pub struct Block {
    pub inner: Vec<BlockElement>
}

impl Block {
    pub fn new(val: &Json) -> Result<Self> {
        let val = val.as_array().ok_or(CommErr::FormatErr("results"))?;
        let mut inner = Vec::new();
        for val_arr in val.iter() {
            inner.push(BlockElement::new(val_arr)?);
        }

        Ok(Block { inner })
    }
}

impl ImpRequest for Block {
    fn search(module: &Notion, val: Json) -> Result<Self> {
        let response = Request::new(module.path())?.request(RequestMethod::GET, val)?;
        let v = response.get("results").ok_or(CommErr::FormatErr("results"))?;
        Block::new(v)
    }
}

impl FmtDisplay for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::default();
        for block in self.inner.iter() {
            output = output.trim_end().to_string() + "\n" + &block.to_string();
        }

        write!(f, "{}", output.trim())
    }
}

impl Default for Block {
    fn default() -> Self {
        Block { inner: Vec::new() }
    }
}
