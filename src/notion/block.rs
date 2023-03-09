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

    fn break_line(&self, next_line: Option<&BlockElement>) -> String {
        use BlockType::*;
        match &self.line_type {
            ToDo|NumberedListItem|BulletedListItem => {
                match next_line {
                    Some(be) => {
                        match be.line_type {
                            ToDo|NumberedListItem|BulletedListItem => "\n".to_string(),
                            _ => "\n\n".to_string()
                        }
                    },
                    None => String::default()
                }
            },
            _ => "\n\n".to_string(),
        }
    }

    fn child_break_line(&self) -> String {
        use BlockType::*;
        match &self.line_type {
            Heading1|Heading2|Heading3|Paragraph => "\n\n".to_string(),
            _ => "\n".to_string(),
        }
    }

    fn special_break_line(&self, content: String) -> String {
        match self.line_type {
            BlockType::Callout|BlockType::Quote => content.replace("\n", "<br/>"),
            _ => content,
        }
    }

    fn get_status_str(&self) -> String {
        let empty_val = Json::default();
        let status_to_replace = if self.status.is_null() {
            ""
        } else if self.status.is_boolean() {
            if self.status.as_bool().unwrap() { "x" } else { " " }
        } else if self.status.is_object() {
            get_property_value(&self.status, None).unwrap_or(&empty_val).as_str().unwrap_or_default()
        } else {
            self.status.as_str().unwrap()
        };

        status_to_replace.to_string()
    }

    fn child_indent(&self, paragraph: String, child_paragraph: String) -> String {
        {
            use BlockType::*;
            match self.line_type {
                Toggle => paragraph.replace("{child}", &child_paragraph),
                Quote => format!("{}{}{}", paragraph, "\n>", child_paragraph),
                Heading1|Heading2|Heading3|Paragraph => format!("{}{}{}", paragraph, "\n\n\t", child_paragraph.replace("\n", "\n\t")),
                _ => format!("{}{}{}", paragraph, "\n\t", child_paragraph.replace("\n", "\n\t")),
            }
        }
    }
}

impl FmtDisplay for BlockElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut paragraph = String::default();
        if !self.line_type.be_empty() && self.line.is_empty() {
            return write!(f, "<br/>");
        } else {
            for text in self.line.iter() {
                paragraph = format!("{}{}", paragraph, text.to_string());
            }
        }

        paragraph = self.line_type.get_str("md").unwrap()
            .replace("{}", &paragraph)
            .replace("{status}", &self.get_status_str());
        paragraph = self.special_break_line(paragraph);

        if !self.child.is_empty() {
            let mut child_paragraph = String::default();
            for child in self.child.iter() {
                child_paragraph = format!("{}{}{}", child_paragraph, child.to_string(), child.child_break_line());
            }
            child_paragraph = child_paragraph.trim_end().to_string();

            paragraph = self.child_indent(paragraph, child_paragraph);
            paragraph = paragraph.trim_end().to_string();
        }

        paragraph = match self.color {
            AnnoColor::Default => paragraph,
            _ => {
                let color_format = Annotation::Color(AnnoColor::Default).get_str("md").unwrap();
                color_format.replace("{}", &paragraph).replace("{color}", self.color.get_str("md").unwrap())
            },
        };

        write!(f, "{}", &paragraph)
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
        for (key,block) in self.inner.iter().enumerate() {
            output = format!("{}{}{}", output, block.to_string(), block.break_line(self.inner.get(key+1)));
        }

        write!(f, "{}", output.trim())
    }
}

impl Default for Block {
    fn default() -> Self {
        Block { inner: Vec::new() }
    }
}
