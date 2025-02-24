mod tests;
pub mod parsing;
pub mod output;
pub mod reference_doc;

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

pub trait RemoveErrors {
    fn remove_errors(&mut self);
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
    Heading(Heading),
    Link(Link),
    Nav(Nav),
    List(List),
    Paragraph(Paragraph),
    Section(Section),
}

impl RemoveErrors for Doc {
    fn remove_errors(&mut self) {
        self.props.remove_errors();
        self.items.retain(|i| !matches!(i, DocItem::Code(Err(_))));
        for item in &mut self.items {
            item.remove_errors();
        }
    }
}

impl RemoveErrors for DocItem {
    fn remove_errors(&mut self) {
        match self {
            DocItem::MText(mtext) => mtext.remove_errors(),
            DocItem::Emphasis(em) => em.remove_errors(),
            DocItem::Code(Ok(code)) => code.remove_errors(),
            DocItem::Heading(head) => head.remove_errors(),
            DocItem::Link(link) => link.remove_errors(),
            DocItem::Nav(nav) => nav.remove_errors(),
            DocItem::List(list) => list.remove_errors(),
            DocItem::Paragraph(par) => par.remove_errors(),
            DocItem::Section(section) => section.remove_errors(),
            _ => {},
        }
    }
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PropValError {
    Int(ParseIntError),
    Date(DateError),
}

impl PropVal {
    fn is_error(&self) -> bool {
        matches![self, PropVal::Error(_)]
    }
}

impl Absorb for Props {
    type Other = Self;
    fn absorb(&mut self, other: Self::Other) {
        for prop in other {
            insert_prop(self, prop)
        }
    }
}

impl RemoveErrors for Props {
    fn remove_errors(&mut self) {
        self.retain(|_, v| !v.is_error());
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

impl RemoveErrors for Section {
    fn remove_errors(&mut self) {
        self.props.remove_errors();
        for item in &mut self.items {
            item.remove_errors();
        }
    }
}

impl RemoveErrors for SectionItem {
    fn remove_errors(&mut self) {
        match self {
            Self::Paragraph(par) => par.remove_errors(),
            Self::Section(section) => section.remove_errors(),
        }
    }
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

impl RemoveErrors for Heading {
    fn remove_errors(&mut self) {
        self.props.remove_errors();
        for item in &mut self.items {
            item.remove_errors();
        }
    }
}

impl RemoveErrors for HeadingItem {
    fn remove_errors(&mut self) {
        if let Self::Em(em) = self {
            em.remove_errors();
        }
    }
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
    Link(Link),
    List(List),
}

impl RemoveErrors for Paragraph {
    fn remove_errors(&mut self) {
        self.props.remove_errors();
        for item in &mut self.items {
            item.remove_errors();
        }
    }
}

impl RemoveErrors for ParagraphItem {
    fn remove_errors(&mut self) {
        match self {
            Self::MText(mtext) => mtext.remove_errors(),
            Self::Em(em) => em.remove_errors(),
            Self::Code(Ok(code)) => code.remove_errors(),
            Self::Link(link) => link.remove_errors(),
            Self::List(list) => list.remove_errors(),
            _ => (),
        }
    }
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

impl RemoveErrors for Emphasis {
    fn remove_errors(&mut self) {
        self.props.remove_errors();
    }
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

impl RemoveErrors for List {
    fn remove_errors(&mut self) {
        self.props.remove_errors();
        for item in &mut self.items {
            item.remove_errors();
        }
    }
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

impl RemoveErrors for Nav {
    fn remove_errors(&mut self) {
        for snav in self {
            snav.remove_errors();
        }
    }
}

impl RemoveErrors for SNav {
    fn remove_errors(&mut self) {
        self.props.remove_errors();
        for link in &mut self.links {
            link.remove_errors();
        }
        for sub in &mut self.subs {
            sub.remove_errors();
        }
    }
}

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Link {
    pub url: String,
    pub items: Vec<LinkItem>,
    pub tags: Tags,
    pub props: Props,
}

impl RemoveErrors for Link {
    fn remove_errors(&mut self) {
        self.props.remove_errors();
        for item in &mut self.items {
            item.remove_errors();
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LinkItem {
    String(String),
    Em(Emphasis),
}

impl RemoveErrors for LinkItem {
    fn remove_errors(&mut self) {
        if let Self::Em(em) = self {
            em.remove_errors();
        }
    }
}

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct CodeBlock {
    pub language: String,
    pub mode: CodeModeHint,
    pub code: String,
    pub tags: Tags,
    pub props: Props,
}

impl RemoveErrors for CodeBlock {
    fn remove_errors(&mut self) {
        self.props.remove_errors();
    }
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum CodeModeHint {
    #[default] Show,
    Runnable,
    Run,
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

impl RemoveErrors for TextWithMeta {
    fn remove_errors(&mut self) {
        self.props.remove_errors();
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

