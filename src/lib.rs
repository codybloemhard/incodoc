mod tests;
pub mod parsing;
pub mod output;
pub mod actions;
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
    /// Navigation.
    Nav(Nav),
    Paragraph(Paragraph),
    Section(Section),
}

/// Tags metadata. Each tag is a string.
pub type Tags = HashSet<String>;

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

fn insert_prop(props: &mut Props, (k, v): (String, PropVal)) {
    let mut insert = true;
    if v.is_error() && let Some(ov) = props.get(&k) && !ov.is_error() {
        insert = false;
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

/// Navigation structure has a description, sub-navigation structures and links to navigate to.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Nav {
    pub description: String,
    pub subs: Vec<Nav>,
    pub links: Vec<Link>,
    pub tags: Tags,
    pub props: Props,
}

/// Links are pieces of text with an accompanying URL.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Link {
    pub url: String,
    pub items: Vec<LinkItem>,
    pub tags: Tags,
    pub props: Props,
}

/// Links have an exterior of plain text that may be emphasised.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LinkItem {
    String(String),
    Em(Emphasis),
}

/// `CodeBlock` contains computer code.
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
            .and_then(|m| if m == 0 { Err(DateError::MonthRange(d)) } else { Ok(m) } )?;
        let day: u8 = d.try_into()
            .map_err(|_| DateError::DayRange(d))
            .and_then(|d| if d == 0 { Err(DateError::DayRange(u64::from(d))) } else { Ok(d) } )?;
        if month > 12 { return Err(DateError::MonthRange(m)); }
        if day > 31 { return Err(DateError::DayRange(d)); }
        Ok(Self { year, month, day })
    }
}

