use std::str::FromStr;
use strum::EnumProperty;
use strum_macros::{Display as Enumdisplay, EnumString};


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
    #[strum(props(md="![{}]({file})"))]
    Image,
    Template,
    ChildPage,
    ChildDatabase,
    Embed,
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

impl BlockType {
    pub fn be_empty(&self) -> bool {
        match self {
            Self::Divider => true,
            _ => false,
        }
    }
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

    fn get_serial_num(&self) -> usize {
        {
            use Annotation::*;
            match self {
                Code => 0,
                Bold => 1,
                Italic => 2,
                Strikethrough => 3,
                Underline => 4,
                Color(_) => 5,
            }
        }
    }

    pub fn sort(list:& mut Vec<Annotation>) {
        list.sort_by(|x, y| x.get_serial_num().cmp(&y.get_serial_num()));
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

