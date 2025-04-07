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

pub trait DocPartActions {
    fn prune_errors(&mut self);
    fn prune_contentless(&mut self);
    fn squash(&mut self);
    fn is_contentless(&self) -> bool;
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
    Em(Emphasis),
    /// Code or an error.
    Code(Result<CodeBlock, CodeIdentError>),
    Link(Link),
    /// Navigation.
    Nav(Nav),
    List(List),
    Paragraph(Paragraph),
    Section(Section),
}

fn squash_alg_text_case<'a>(
    ltext: &mut Option<&'a mut String>,
    lmtext: &mut Option<&mut TextWithMeta>,
    lem: &mut Option<&mut Emphasis>,
    text: &'a mut String, keep: &mut Vec<bool>
) {
    match ltext {
        None => {
            *ltext = Some(text);
            keep.push(true);
        },
        Some(t) => {
            t.push_str(text);
            keep.push(false);
        },
    }
    *lmtext = None;
    *lem = None;
}

fn squash_alg_mtext_case<'a>(
    ltext: &mut Option<&mut String>,
    lmtext: &mut Option<&'a mut TextWithMeta>,
    lem: &mut Option<&mut Emphasis>,
    mtext: &'a mut TextWithMeta, keep: &mut Vec<bool>
) {
    mtext.prune_contentless();
    if mtext.is_contentless() {
        keep.push(false);
    } else {
        *ltext = None;
        if let Some(t) = lmtext {
            if mtext.tags == t.tags && mtext.props == t.props {
                t.text.push_str(&mtext.text);
                keep.push(false);
            } else {
                *lmtext = Some(mtext);
                keep.push(true);
            }
        } else {
            *lmtext = Some(mtext);
            keep.push(true);
        }
    }
    *lem = None;
}

fn squash_alg_em_case<'a>(
    ltext: &mut Option<&mut String>,
    lmtext: &mut Option<&mut TextWithMeta>,
    lem: &mut Option<&'a mut Emphasis>,
    em: &'a mut Emphasis, keep: &mut Vec<bool>
) {
    *ltext = None;
    *lmtext = None;
    if let Some(e) = lem {
        if em.props == e.props && em.tags == e.tags
            && em.strength == e.strength && em.etype == e.etype {
            e.text.push_str(&em.text);
            keep.push(false);
        } else {
            *lem = Some(em);
            keep.push(true);
        }
    } else {
        *lem = Some(em);
        keep.push(true);
    }
}

fn squash_alg_def_case(
    ltext: &mut Option<&mut String>,
    lmtext: &mut Option<&mut TextWithMeta>,
    lem: &mut Option<&mut Emphasis>,
    keep: &mut Vec<bool>
) {
    keep.push(true);
    *ltext = None;
    *lmtext = None;
    *lem = None;
}

macro_rules! impl_squash_text_mtext_em {
    ($enumname:ident) => {
        fn squash(&mut self) {
            let mut keep = Vec::new();
            let mut ltext: Option<&mut String> = None;
            let mut lmtext: Option<&mut TextWithMeta> = None;
            let mut lem: Option<&mut Emphasis> = None;

            // squash items into earlier item if possible
            for item in &mut self.items {
                match item {
                    $enumname::Text(text) => {
                        squash_alg_text_case(&mut ltext, &mut lmtext, &mut lem, text, &mut keep);
                    },
                    $enumname::MText(mtext) => {
                        squash_alg_mtext_case(&mut ltext, &mut lmtext, &mut lem, mtext, &mut keep);
                    },
                    $enumname::Em(em) => {
                        squash_alg_em_case(&mut ltext, &mut lmtext, &mut lem, em, &mut keep);
                    },
                    _ => {
                        squash_alg_def_case(&mut ltext, &mut lmtext, &mut lem, &mut keep);
                    },
                }
            }

            // remove parts that have been squashed
            let mut kiter = keep.into_iter();
            self.items.retain(|_| kiter.next().unwrap());

            // downgrade MText's with no meta
            for item in &mut self.items {
                if let $enumname::MText(TextWithMeta { text, tags, props }) = item {
                    if tags.is_empty() && props.is_empty() {
                        *item = $enumname::Text(std::mem::take(text));
                    }
                }
            }

            // recurse
            for item in &mut self.items {
                item.squash();
            }
        }
    }
}

macro_rules! impl_squash_text_em {
    ($enumname:ident) => {
        fn squash(&mut self) {
            let mut keep = Vec::new();
            let mut ltext: Option<&mut String> = None;
            let mut lem: Option<&mut Emphasis> = None;

            // squash items into earlier item if possible
            for item in &mut self.items {
                match item {
                    $enumname::String(text) => {
                        squash_alg_text_case(&mut ltext, &mut None, &mut lem, text, &mut keep);
                    },
                    $enumname::Em(em) => {
                        squash_alg_em_case(&mut ltext, &mut None, &mut lem, em, &mut keep);
                    },
                }
            }

            // remove parts that have been squashed
            let mut kiter = keep.into_iter();
            self.items.retain(|_| kiter.next().unwrap());
        }
    }
}

impl DocPartActions for Doc {
    fn prune_errors(&mut self) {
        self.props.prune_errors();
        self.items.retain(|i| !matches!(i, DocItem::Code(Err(_))));
        for item in &mut self.items {
            item.prune_errors();
        }
    }

    fn prune_contentless(&mut self) {
        self.tags.prune_contentless();
        self.props.prune_contentless();
        for item in &mut self.items {
            item.prune_contentless();
        }
        self.items.retain(|item| !item.is_contentless());
    }

    impl_squash_text_mtext_em!(DocItem);

    fn is_contentless(&self) -> bool {
        self.items.is_empty()
    }
}

impl DocPartActions for DocItem {
    fn prune_errors(&mut self) {
        match self {
            DocItem::MText(mtext) => mtext.prune_errors(),
            DocItem::Em(em) => em.prune_errors(),
            DocItem::Code(Ok(code)) => code.prune_errors(),
            DocItem::Link(link) => link.prune_errors(),
            DocItem::Nav(nav) => nav.prune_errors(),
            DocItem::List(list) => list.prune_errors(),
            DocItem::Paragraph(par) => par.prune_errors(),
            DocItem::Section(section) => section.prune_errors(),
            _ => { },
        }
    }

    fn prune_contentless(&mut self) {
        match self {
            DocItem::Text(text) => text.prune_contentless(),
            DocItem::MText(mtext) => mtext.prune_contentless(),
            DocItem::Em(em) => em.prune_contentless(),
            DocItem::Code(Ok(code)) => code.prune_contentless(),
            DocItem::Code(Err(_)) => { },
            DocItem::Link(link) => link.prune_contentless(),
            DocItem::Nav(nav) => nav.prune_contentless(),
            DocItem::List(list) => list.prune_contentless(),
            DocItem::Paragraph(par) => par.prune_contentless(),
            DocItem::Section(section) => section.prune_contentless(),
        }
    }

    fn squash(&mut self) {
        match self {
            DocItem::Link(link) => link.squash(),
            DocItem::Nav(nav) => nav.squash(),
            DocItem::List(list) => list.squash(),
            DocItem::Paragraph(par) => par.squash(),
            _ => { },
        }
    }

    fn is_contentless(&self) -> bool {
        match self {
            DocItem::Text(text) => text.is_empty(),
            DocItem::MText(mtext) => mtext.is_contentless(),
            DocItem::Em(em) => em.is_contentless(),
            DocItem::Code(Ok(code)) => code.is_contentless(),
            DocItem::Code(Err(_)) => true,
            DocItem::Link(link) => link.is_contentless(),
            DocItem::Nav(nav) => nav.is_contentless(),
            DocItem::List(list) => list.is_contentless(),
            DocItem::Paragraph(par) => par.is_contentless(),
            DocItem::Section(section) => section.is_contentless(),
        }
    }
}

impl DocPartActions for String {
    fn prune_errors(&mut self) { }

    fn prune_contentless(&mut self) {
        let trimmed = self.trim();
        if trimmed.is_empty() {
            *self = trimmed.to_string();
        }
    }

    fn squash(&mut self) { }

    fn is_contentless(&self) -> bool {
        self.is_empty()
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

impl DocPartActions for Tags {
    fn prune_errors(&mut self) { }

    fn prune_contentless(&mut self) {
        self.retain(|t|
            !t.is_empty() &&
            !t.chars().all(|c| c.is_whitespace())
        );
    }

    fn squash(&mut self) { }

    fn is_contentless(&self) -> bool {
        self.is_empty()
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

impl DocPartActions for Props {
    fn prune_errors(&mut self) {
        self.retain(|_, v| !v.is_error());
    }

    fn prune_contentless(&mut self) {
        for pval in self.values_mut() {
            pval.prune_contentless();
        }
        self.retain(|k, v|
            !k.is_empty() && !v.is_contentless() &&
            !k.chars().all(|c| c.is_whitespace())
        );
    }

    fn squash(&mut self) { }

    fn is_contentless(&self) -> bool {
        self.is_empty()
    }
}

impl DocPartActions for PropVal {
    fn prune_errors(&mut self) {  }

    fn prune_contentless(&mut self) {
        match self {
            Self::String(string) => string.prune_contentless(),
            Self::Text(string) => string.prune_contentless(),
            _ => { },
        }
    }

    fn squash(&mut self) { }

    fn is_contentless(&self) -> bool {
        match self {
            Self::String(string) => string.is_empty(),
            Self::Text(string) => string.is_empty(),
            _ => false,
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

impl DocPartActions for Section {
    fn prune_errors(&mut self) {
        self.props.prune_errors();
        for item in &mut self.items {
            item.prune_errors();
        }
    }

    fn prune_contentless(&mut self) {
        self.heading.prune_contentless();
        for item in &mut self.items {
            item.prune_contentless();
        }
        self.items.retain(|item| !item.is_contentless());
        self.tags.prune_contentless();
        self.props.prune_contentless();
    }

    fn squash(&mut self) {
        for item in &mut self.items {
            item.squash();
        }
    }

    fn is_contentless(&self) -> bool {
        self.heading.is_contentless() && self.items.is_empty()
    }
}

impl DocPartActions for SectionItem {
    fn prune_errors(&mut self) {
        match self {
            Self::Paragraph(par) => par.prune_errors(),
            Self::Section(section) => section.prune_errors(),
        }
    }

    fn prune_contentless(&mut self) {
        match self {
            Self::Paragraph(par) => par.prune_contentless(),
            Self::Section(section) => section.prune_contentless(),
        }
    }

    fn squash(&mut self) {
        match self {
            Self::Paragraph(par) => par.squash(),
            Self::Section(section) => section.squash(),
        }
    }

    fn is_contentless(&self) -> bool {
        match self {
            Self::Paragraph(par) => par.is_contentless(),
            Self::Section(section) => section.is_contentless(),
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

impl DocPartActions for Heading {
    fn prune_errors(&mut self) {
        self.props.prune_errors();
        for item in &mut self.items {
            item.prune_errors();
        }
    }

    fn prune_contentless(&mut self) {
        for item in &mut self.items {
            item.prune_contentless();
        }
        self.items.retain(|item| !item.is_contentless());
        self.tags.prune_contentless();
        self.props.prune_contentless();
    }

    impl_squash_text_em!(HeadingItem);

    fn is_contentless(&self) -> bool {
        self.items.is_empty()
    }
}

impl DocPartActions for HeadingItem {
    fn prune_errors(&mut self) {
        if let Self::Em(em) = self {
            em.prune_errors();
        }
    }

    fn prune_contentless(&mut self) {
        match self {
            Self::String(string) => string.prune_contentless(),
            Self::Em(em) => em.prune_contentless(),
        }
    }

    fn squash(&mut self) { }

    fn is_contentless(&self) -> bool {
        match self {
            Self::String(string) => string.is_empty(),
            Self::Em(em) => em.is_contentless(),
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

impl DocPartActions for Paragraph {
    fn prune_errors(&mut self) {
        self.props.prune_errors();
        for item in &mut self.items {
            item.prune_errors();
        }
    }

    fn prune_contentless(&mut self) {
        for item in &mut self.items {
            item.prune_contentless();
        }
        self.items.retain(|item| !item.is_contentless());
        self.tags.prune_contentless();
        self.props.prune_contentless();
    }

    impl_squash_text_mtext_em!(ParagraphItem);

    fn is_contentless(&self) -> bool {
        self.items.is_empty()
    }
}

impl DocPartActions for ParagraphItem {
    fn prune_errors(&mut self) {
        match self {
            Self::MText(mtext) => mtext.prune_errors(),
            Self::Em(em) => em.prune_errors(),
            Self::Code(Ok(code)) => code.prune_errors(),
            Self::Link(link) => link.prune_errors(),
            Self::List(list) => list.prune_errors(),
            _ => (),
        }
    }

    fn prune_contentless(&mut self) {
        match self {
            Self::Text(text) => text.prune_contentless(),
            Self::MText(mtext) => mtext.prune_contentless(),
            Self::Em(em) => em.prune_contentless(),
            Self::Code(Ok(code)) => code.prune_contentless(),
            Self::Code(Err(_)) => { },
            Self::Link(link) => link.prune_contentless(),
            Self::List(list) => list.prune_contentless(),
        }
    }

    fn squash(&mut self) {
        match self {
            Self::Link(link) => link.squash(),
            Self::List(list) => list.squash(),
            _ => { },
        }
    }

    fn is_contentless(&self) -> bool {
        match self {
            Self::Text(text) => text.is_empty(),
            Self::MText(mtext) => mtext.is_contentless(),
            Self::Em(em) => em.is_contentless(),
            Self::Code(Ok(code)) => code.is_contentless(),
            Self::Code(Err(_)) => true,
            Self::Link(link) => link.is_contentless(),
            Self::List(list) => list.is_contentless(),
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

impl DocPartActions for Emphasis {
    fn prune_errors(&mut self) {
        self.props.prune_errors();
    }

    fn prune_contentless(&mut self) {
        self.text.prune_contentless();
        self.tags.prune_contentless();
        self.props.prune_contentless();
    }

    fn squash(&mut self) { }

    fn is_contentless(&self) -> bool {
        self.text.is_empty()
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

impl DocPartActions for List {
    fn prune_errors(&mut self) {
        self.props.prune_errors();
        for par in &mut self.items {
            par.prune_errors();
        }
    }

    fn prune_contentless(&mut self) {
        for par in &mut self.items {
            par.prune_contentless();
        }
        self.items.retain(|par| !par.is_contentless());
        self.tags.prune_contentless();
        self.props.prune_contentless();
    }

    fn squash(&mut self) {
        for item in &mut self.items {
            item.squash();
        }
    }

    fn is_contentless(&self) -> bool {
        self.items.is_empty()
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

impl DocPartActions for Nav {
    fn prune_errors(&mut self) {
        for snav in self {
            snav.prune_errors();
        }
    }

    fn prune_contentless(&mut self) {
        for snav in self.iter_mut() {
            snav.prune_contentless();
        }
        self.retain(|snav| !snav.is_contentless());
    }

    fn squash(&mut self) { }

    fn is_contentless(&self) -> bool {
        self.is_empty()
    }
}

impl DocPartActions for SNav {
    fn prune_errors(&mut self) {
        self.props.prune_errors();
        for link in &mut self.links {
            link.prune_errors();
        }
        for sub in &mut self.subs {
            sub.prune_errors();
        }
    }

    fn prune_contentless(&mut self) {
        self.description.prune_contentless();
        for link in &mut self.links {
            link.prune_contentless();
        }
        self.links.retain(|link| !link.is_contentless());
        for sub in &mut self.subs {
            sub.prune_contentless();
        }
        self.subs.retain(|sub| !sub.is_contentless());
        self.tags.prune_contentless();
        self.props.prune_contentless();
    }

    fn squash(&mut self) { }

    fn is_contentless(&self) -> bool {
        self.subs.is_empty() && self.links.is_empty()
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

impl DocPartActions for Link {
    fn prune_errors(&mut self) {
        self.props.prune_errors();
        for item in &mut self.items {
            item.prune_errors();
        }
    }

    fn prune_contentless(&mut self) {
        self.url.prune_contentless();
        for item in &mut self.items {
            item.prune_contentless();
        }
        self.items.retain(|item| !item.is_contentless());
        self.tags.prune_contentless();
        self.props.prune_contentless();
    }

    impl_squash_text_em!(LinkItem);

    fn is_contentless(&self) -> bool {
        self.url.is_empty() && self.items.is_empty()
    }
}

/// Links have an exterior of plain text that may be emphasised.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LinkItem {
    String(String),
    Em(Emphasis),
}

impl DocPartActions for LinkItem {
    fn prune_errors(&mut self) {
        if let Self::Em(em) = self {
            em.prune_errors();
        }
    }

    fn prune_contentless(&mut self) {
        match self {
            Self::String(string) => string.prune_contentless(),
            Self::Em(em) => em.prune_contentless(),
        }
    }

    fn squash(&mut self) { }

    fn is_contentless(&self) -> bool {
        match self {
            Self::String(string) => string.is_empty(),
            Self::Em(em) => em.is_contentless(),
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

impl DocPartActions for CodeBlock {
    fn prune_errors(&mut self) {
        self.props.prune_errors();
    }

    fn prune_contentless(&mut self) {
        self.language.prune_contentless();
        self.code.prune_contentless();
        self.tags.prune_contentless();
        self.props.prune_contentless();
    }

    fn squash(&mut self) { }

    fn is_contentless(&self) -> bool {
        self.code.is_empty()
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

impl DocPartActions for TextWithMeta {
    fn prune_errors(&mut self) {
        self.props.prune_errors();
    }

    fn prune_contentless(&mut self) {
        self.text.prune_contentless();
        self.tags.prune_contentless();
        self.props.prune_contentless();
    }

    fn squash(&mut self) { }

    fn is_contentless(&self) -> bool {
        self.text.is_empty()
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

