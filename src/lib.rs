mod tests;
pub mod parsing;
pub mod output;
pub mod reference_doc;

use std::{
    num::ParseIntError,
    collections::{ HashMap, HashSet },
};

use pest_derive::Parser;

/// Parser for incodoc.
#[derive(Parser)]
#[grammar = "parse/incodoc.pest"]
pub struct IncodocParser;

/// Merge two objects by having one absorb the other.
pub trait Absorb {
    type Other;
    /// Absorb other into self.
    fn absorb(&mut self, other: Self::Other);
}

/// Remove errors from self.
pub trait RemoveErrors {
    fn remove_errors(&mut self);
}

/// Document.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Doc {
    pub tags: Tags,
    pub props: Props,
    pub items: Vec<DocItem>,
}

/// Document item.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DocItem {
    Text(String),
    /// Text with meta attached.
    MText(TextWithMeta),
    Emphasis(Emphasis),
    /// Code or an error.
    Code(Result<CodeBlock, CodeIdentError>),
    Link(Link),
    /// Navigation.
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
            DocItem::Link(link) => link.remove_errors(),
            DocItem::Nav(nav) => nav.remove_errors(),
            DocItem::List(list) => list.remove_errors(),
            DocItem::Paragraph(par) => par.remove_errors(),
            DocItem::Section(section) => section.remove_errors(),
            _ => {},
        }
    }
}

/// Tags metadata. Each tag is a string.
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

/// Properties metadata. Each property is a tuple of an identifier and a value.
pub type Props = HashMap<String, PropVal>;

/// Value properties can take.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PropVal {
    String(String),
    Text(String),
    Int(i64),
    Date(Date),
    Error(PropValError),
}

/// Error when no valid value could be parsed.
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

/// A section is a heading followed by content that goes with it.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Section {
    pub heading: Heading,
    pub items: Vec<SectionItem>,
    pub tags: Tags,
    pub props: Props,
}

/// Section items are either paragraphs or sub-sections.
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

/// Heading, a title for the accompanying content.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Heading {
    pub level: u8,
    pub items: Vec<HeadingItem>,
    pub tags: Tags,
    pub props: Props,
}

/// Headings are plain text that can have emphasis.
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

/// Paragraph is a grouping of content.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Paragraph {
    pub items: Vec<ParagraphItem>,
    pub tags: Tags,
    pub props: Props,
}

/// Paragraphs can have content and further structure but no smaller sub-sections.
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

/// Emphasised or de-emphasised piece of text.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Emphasis {
    pub strength: EmStrength,
    pub etype: EmType,
    pub text: String,
    pub tags: Tags,
    pub props: Props,
}

/// Degree of emphasis or de-emphasis.
#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum EmStrength {
    #[default]
    Light,
    Medium,
    Strong,
}

/// Whether it is an emphasis or de-emphasis.
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

/// Lists are fine-grained structure in a document.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct List {
    pub ltype: ListType,
    pub items: Vec<Paragraph>,
    pub tags: Tags,
    pub props: Props,
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ListType {
    /// Each item has a distinct denotation.
    Distinct,
    /// Each item is denotated identically.
    #[default] Identical,
    /// Each item is either checked off or not.
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

/// Navigation structure contains sub-navigation structures.
pub type Nav = Vec<SNav>;

/// Sub-navigation structure has a description, sub-navigation structures and links to navigate to.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct SNav {
    pub description: String,
    pub subs: Vec<SNav>,
    pub links: Vec<Link>,
    pub tags: Tags,
    pub props: Props,
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

/// Links are pieces of text with an accompanying URL.
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

/// Links have an exterior of plain text that may be emphasised.
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

/// CodeBlock contains computer code.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct CodeBlock {
    /// Computer language in which the code is written.
    pub language: String,
    /// Behavioural hint.
    pub mode: CodeModeHint,
    /// The code.
    pub code: String,
    pub tags: Tags,
    pub props: Props,
}

impl RemoveErrors for CodeBlock {
    fn remove_errors(&mut self) {
        self.props.remove_errors();
    }
}

/// Behavioural hint: the block hints what to do with the code.
#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum CodeModeHint {
    /// Hint to show the code in the document.
    #[default] Show,
    /// Hint to show the code in the document and to signal that it is supposed to be able to run.
    Runnable,
    /// Hint to show the code in the document and to run the code and show the results as well.
    Run,
    /// Hint to run the code and show the results in the document instead of the code itself.
    Replace,
}

/// Text that has metadata: tags and/or properties.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct TextWithMeta {
    pub text: String,
    pub tags: Tags,
    pub props: Props,
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

/// Error to signal that the code was not formatted properly.
#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct CodeIdentError;

/// Simple date: it is not checked if it actually exists on the calendar.
#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Date {
    pub year: i16,
    pub month: u8,
    pub day: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DateError {
    /// The year was out of range of i16.
    YearRange(i64),
    /// The month was out of range: bigger than 12.
    MonthRange(u64),
    /// The day was out of range: bigger than 31.
    DayRange(u64),
    /// There was an error parsing an integer in the date.
    Parsing(ParseIntError),
}

impl Date {
    pub fn new(y: i64, m: u64, d: u64) -> Result<Self, DateError> {
        let year: i16 = y.try_into().map_err(|_| DateError::YearRange(y))?;
        let month: u8 = m.try_into()
            .map_err(|_| DateError::MonthRange(m))
            .and_then(|m| if m == 0 { Err(DateError::MonthRange(m as u64)) } else { Ok(m) } )?;
        let day: u8 = d.try_into()
            .map_err(|_| DateError::DayRange(d))
            .and_then(|d| if d == 0 { Err(DateError::DayRange(d as u64)) } else { Ok(d) } )?;
        if month > 12 { return Err(DateError::MonthRange(m)); }
        if day > 31 { return Err(DateError::DayRange(d)); }
        Ok(Self { year, month, day })
    }
}

