use std::str::FromStr;
use std::fmt::Display as FmtDisplay;
use anyhow::Result;
use strum::EnumProperty;
use serde_json::Map;

use super::{Notion, CommErr, get_value_str, get_property_value, Json, NewImp, text::*};


#[derive(Debug)]
pub struct FragmentText {
    pub text: String,
    pub href: String,
    pub annotation: Vec<Annotation>,
}

impl FragmentText {
    pub fn new(val: &Json) -> Result<Self> {
        let mut annotation: Vec<Annotation> = Vec::new();
        let default_map = Map::new();
        let anno = val.get("annotations").ok_or(CommErr::FormatErr("annotations"))?.as_object().unwrap_or(&default_map);
        for (anno_key, anno_val) in anno.iter() {
            if anno_key == "color" {
                annotation.push(Annotation::from_str(anno_key).unwrap().reset_val(anno_val.as_str().unwrap()));
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

        Annotation::sort(&mut annotation);
        Ok(FragmentText  {
            text: get_value_str(val, "plain_text")?,
            href: val.get("href").unwrap_or(&Json::default()).as_str().unwrap_or_default().to_string(),
            annotation
        })
    }
}

impl FmtDisplay for FragmentText  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut anno_format = "{}".to_string();
        let mut conflict = false;
        for anno in self.annotation.iter() {
            anno_format = match anno {
                Annotation::Color(AnnoColor::Default) => anno_format,
                Annotation::Color(c) => (&anno_format).replace("{}", anno.get_str("md").unwrap()).replace("{color}", c.get_str("md").unwrap()),
                Annotation::Bold|Annotation::Italic => {
                    let anno_prop = if !conflict { conflict = true; "md" } else { conflict=false;"mdrpl" };
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
    pub line: Vec<FragmentText>,
    pub line_type: BlockType,
    pub color: AnnoColor,
    pub child: Vec<BlockElement>,
    pub status: Json,
}

impl BlockElement {
    fn from_type(line_type: BlockType) -> Self {
        BlockElement { line: Vec::new(), line_type, color: AnnoColor::Default, child: Vec::new(), status: Json::default() }
    }

    fn from_text(line_type: BlockType, text: String) -> Self {
        BlockElement {
            line: vec![ FragmentText { text, href: String::default(), annotation: Vec::new() } ],
            line_type,
            color: AnnoColor::default(),
            child: Vec::new(),
            status: Json::default(),
        }
    }

    pub fn new(value: &Json) -> Result<Self> {
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

        let mut line: Vec<FragmentText> = Vec::new();
        for v in rich_text.iter() {
            line.push(FragmentText::new(v)?);
        }

        let color  = AnnoColor::from_str(&get_value_str(block, "color").unwrap_or_default()).unwrap_or_default();

        // TODO: 异步
        let mut child = Vec::new();
        if value.get("has_children")
            .ok_or(CommErr::FormatErr("has_children"))?
            .as_bool().ok_or(CommErr::FormatErr("has_children"))?
        {
            let block = Notion::Blocks(get_value_str(value, "id")?).search::<Block>()?;
            for be in block.inner.into_iter() {
                child.push(be);
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
            return write!(f, "<br/>");
        } else {
            for text in self.line.iter() {
                paragraph += &text.to_string();
            }
        }

        let empty_val = Json::default();
        let format = self.line_type.get_str("md").unwrap();
        let status_to_replace = if self.status.is_null() {
            ""
        } else if self.status.is_boolean() {
            if self.status.as_bool().unwrap() { "x" } else { " " }
        } else if self.status.is_object() {
            get_property_value(&self.status, None).unwrap_or(&empty_val).as_str().unwrap_or_default()
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

        if !self.child.is_empty() {
            let mut child_paragraph = String::default();
            for child in self.child.iter() {
                child_paragraph = child_paragraph + &child.to_string();
            }
            child_paragraph = child_paragraph.trim_start().to_string();

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

impl NewImp for Block {
    fn new(val: &Json) -> Result<Self> {
        let val = val.as_array().ok_or(CommErr::FormatErr("results"))?;
        let mut inner = Vec::new();
        for val_arr in val.iter() {
            inner.push(BlockElement::new(val_arr)?);
        }

        Ok(Block { inner })
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
