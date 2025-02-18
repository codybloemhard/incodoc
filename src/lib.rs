mod tests;
pub mod parsing;
pub mod output;

use std::{
    num::ParseIntError,
    collections::{ HashMap, HashSet },
};

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parse/incodoc.pest"]
pub struct IncodocParser;

pub trait Absorb {
    type Other;
    fn absorb(&mut self, other: Self::Other);
}

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Doc {
    tags: Tags,
    props: Props,
    items: Vec<DocItem>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DocItem {
    Text(String),
    MText(TextWithMeta),
    Emphasis(Emphasis),
    Code(Result<CodeBlock, CodeIdentError>),
    Paragraph(Paragraph),
    Heading(Heading),
    List(List),
    Section(Section),
    Link(Link),
    Nav(Nav),
}

pub type Tags = HashSet<String>;

impl Absorb for Tags {
    type Other = Option<Self>;
    fn absorb(&mut self, other: Self::Other) {
        if let Some(o) = other {
            for v in o {
                self.insert(v);
            }
        }
    }
}

pub type Props = HashMap<String, PropVal>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PropVal {
    String(String),
    Text(String),
    Int(i64),
    Date(Date),
    Error(PropValError),
}

impl PropVal {
    fn is_error(&self) -> bool {
        matches![self, PropVal::Error(_)]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PropValError {
    Int(ParseIntError),
    Date(DateError),
}

impl Absorb for Props {
    type Other = Self;
    fn absorb(&mut self, other: Self::Other) {
        for prop in other {
            insert_prop(self, prop)
        }
    }
}

fn insert_prop(props: &mut Props, (k, v): (String, PropVal)) {
    let mut insert = true;
    if v.is_error() {
        if let Some(ov) = props.get(&k) {
            if !ov.is_error() {
                insert = false;
            }
        }
    }
    if insert {
        props.insert(k, v);
    }
}

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Section {
    heading: Heading,
    items: Vec<SectionItem>,
    tags: Tags,
    props: Props,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SectionItem {
    Paragraph(Paragraph),
    Section(Section),
}

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Heading {
    level: u8,
    items: Vec<HeadingItem>,
    tags: Tags,
    props: Props,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HeadingItem {
    String(String),
    Em(Emphasis),
}

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Paragraph {
    items: Vec<ParagraphItem>,
    tags: Tags,
    props: Props,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParagraphItem {
    Text(String),
    MText(TextWithMeta),
    Em(Emphasis),
    Code(Result<CodeBlock, CodeIdentError>),
    List(List),
    Link(Link),
}

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Emphasis {
    strength: EmStrength,
    etype: EmType,
    text: String,
    tags: Tags,
    props: Props,
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum EmStrength {
    #[default]
    Light,
    Medium,
    Strong,
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum EmType {
    #[default]
    Emphasis,
    Deemphasis,
}

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct List {
    ltype: ListType,
    items: Vec<ParagraphItem>,
    tags: Tags,
    props: Props,
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ListType {
    Distinct,
    #[default] Identical,
    Checked,
}

pub type Nav = Vec<SNav>;

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct SNav {
    description: String,
    subs: Vec<SNav>,
    links: Vec<Link>,
    tags: Tags,
    props: Props,
}

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Link {
    pub url: String,
    pub items: Vec<LinkItem>,
    pub tags: Tags,
    pub props: Props,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LinkItem {
    String(String),
    Em(Emphasis),
}

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct CodeBlock {
    pub language: String,
    pub mode: CodeModeHint,
    pub code: String,
    pub tags: Tags,
    pub props: Props,
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum CodeModeHint {
    #[default] Show,
    Choice,
    Auto,
    Replace,
}

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct TextWithMeta {
    text: String,
    tags: Tags,
    props: Props,
}

impl TextWithMeta {
    fn meta_is_empty(&self) -> bool {
        self.tags.is_empty() && self.props.is_empty()
    }
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct CodeIdentError;

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Date {
    year: i16,
    month: u8,
    day: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DateError {
    YearRange(i64),
    MonthRange(u64),
    DayRange(u64),
    Parsing(ParseIntError),
}

impl Date {
    pub fn new(y: i64, m: u64, d: u64) -> Result<Self, DateError> {
        let year: i16 = y.try_into().map_err(|_| DateError::YearRange(y))?;
        let month: u8 = m.try_into().map_err(|_| DateError::MonthRange(m))?;
        let day: u8 = d.try_into().map_err(|_| DateError::DayRange(d))?;
        if month > 12 { return Err(DateError::MonthRange(m)); }
        if day > 31 { return Err(DateError::DayRange(d)); }
        Ok(Self { year, month, day })
    }
}

