mod tests;

use std::{
    num::ParseIntError,
    collections::{ HashMap, HashSet },
};

use pest::{
    Parser,
    iterators::Pair,
};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parse/incodoc.pest"]
pub struct IncodocParser;

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
    Par(Paragraph),
    Heading(Heading),
    List(List),
    Section(Section),
}

pub trait Absorb {
    type Other;
    fn absorb(&mut self, other: Self::Other);
}

pub fn parse(input: &str) -> Result<Doc, String> {
    let mut doc = Doc::default();
    let pairs = match IncodocParser::parse(Rule::top, input) {
        Ok(res) => res,
        Err(e) => return Err(e.to_string()),
    };
    for inner in pairs {
        match inner.as_rule() {
            Rule::tags => doc.tags.absorb(parse_tags(inner)),
            Rule::props => doc.props.absorb(parse_props(inner)),
            Rule::text_item => {
                let text = parse_text_item(inner);
                if text.meta_is_empty() {
                    doc.items.push(DocItem::Text(text.text));
                } else {
                    doc.items.push(DocItem::MText(text));
                }
            },
            Rule::emphasis => doc.items.push(DocItem::Emphasis(parse_emphasis(inner))),
            Rule::heading => doc.items.push(DocItem::Heading(parse_heading(inner))),
            Rule::code => doc.items.push(DocItem::Code(parse_code(inner))),
            Rule::paragraph => doc.items.push(DocItem::Par(parse_paragraph(inner))),
            Rule::list => doc.items.push(DocItem::List(parse_list(inner))),
            Rule::section => doc.items.push(DocItem::Section(parse_section(inner))),
            _ => {},
        }
    }
    Ok(doc)
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

fn parse_tags(pair: Pair<'_, Rule>) -> Option<Tags> {
    let mut res = HashSet::new();
    for strings in pair.into_inner() {
        if matches!(strings.as_rule(), Rule::prop_tuple) {
            return None;
        }
        for string in strings.into_inner() {
            res.insert(parse_string(string));
        }
    }
    Some(res)
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

fn parse_props(pair: Pair<'_, Rule>) -> Props {
    let mut props = HashMap::new();
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::prop_tuple => insert_prop(&mut props, parse_prop_tuple(inner)),
            r => panic!("IP: parse_props: illegal rule: {:?};", r),
        }
    }
    props
}

fn parse_prop_tuple(pair: Pair<'_, Rule>) -> (String, PropVal) {
    let mut inners = pair.into_inner();
    let string = inners.next().expect("IP: parse_prop_tuple: no string;");
    let prop_val = inners.next().expect("IP: parse_prop_tuple: no prop_val;");
    (parse_string(string), parse_prop_val(prop_val))
}

fn parse_prop_val(pair: Pair<'_, Rule>) -> PropVal {
    match pair.as_rule() {
        Rule::string => PropVal::String(parse_string(pair)),
        Rule::text => PropVal::Text(parse_text(pair)),
        Rule::int => match parse_int(pair) {
            Ok(int) => PropVal::Int(int),
            Err(error) => PropVal::Error(PropValError::Int(error)),
        },
        Rule::date => match parse_date(pair) {
            Ok(date) => PropVal::Date(date),
            Err(error) => PropVal::Error(PropValError::Date(error)),
        },
        r => panic!("IP: parse_prop_val: illegal rule: {:?};", r),
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
    Par(Paragraph),
    Section(Section),
}

pub fn parse_section(pair: Pair<'_, Rule>) -> Section {
    let mut iter = pair.into_inner();
    let heading = parse_heading(iter.next().expect("IP: parse_section: no heading"));
    let mut items = Vec::new();
    let mut tags = Tags::default();
    let mut props = Props::default();
    for inner in iter {
        match inner.as_rule() {
            Rule::paragraph => items.push(SectionItem::Par(parse_paragraph(inner))),
            Rule::section => items.push(SectionItem::Section(parse_section(inner))),
            Rule::tags => tags.absorb(parse_tags(inner)),
            Rule::props => props.absorb(parse_props(inner)),
            r => panic!("IP: parse_section: illegal rule: {:?};", r),
        }
    }

    Section {
        heading,
        items,
        tags,
        props,
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
    Text(String),
    MText(TextWithMeta),
    Em(Emphasis),
}

pub fn parse_heading(pair: Pair<'_, Rule>) -> Heading {
    let mut items = Vec::new();
    let mut tags = Tags::default();
    let mut props = Props::default();
    let mut iter = pair.into_inner();
    let level =
        parse_uint_capped(iter.next().expect("IP: parse_heading: no strength;"))
        .min(255) as u8;
    for inner in iter {
        match inner.as_rule() {
            Rule::text_item => {
                let text = parse_text_item(inner);
                if text.meta_is_empty() {
                    items.push(HeadingItem::Text(text.text));
                } else {
                    items.push(HeadingItem::MText(text));
                }
            },
            Rule::emphasis => items.push(HeadingItem::Em(parse_emphasis(inner))),
            Rule::tags => tags.absorb(parse_tags(inner)),
            Rule::props => props.absorb(parse_props(inner)),
            r => panic!("IP: parse_heading: illegal rule: {:?};", r),
        }
    }
    Heading {
        level,
        items,
        tags,
        props,
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
    List(List),
}

pub fn parse_paragraph(pair: Pair<'_, Rule>) -> Paragraph {
    let mut items = Vec::new();
    let mut tags = Tags::default();
    let mut props = Props::default();
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::text_item => {
                let text = parse_text_item(inner);
                if text.meta_is_empty() {
                    items.push(ParagraphItem::Text(text.text));
                } else {
                    items.push(ParagraphItem::MText(text));
                }
            },
            Rule::emphasis => items.push(ParagraphItem::Em(parse_emphasis(inner))),
            Rule::code => items.push(ParagraphItem::Code(parse_code(inner))),
            Rule::list => items.push(ParagraphItem::List(parse_list(inner))),
            Rule::tags => tags.absorb(parse_tags(inner)),
            Rule::props => props.absorb(parse_props(inner)),
            r => panic!("IP: parse_paragraph: illegal rule: {:?};", r),
        }
    }
    Paragraph {
        items,
        tags,
        props,
    }
}

#[derive(Clone, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Emphasis {
    strength: EmStrength,
    etype: EmType,
    text: String,
}

#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum EmStrength {
    Light,
    Medium,
    Strong,
}

#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum EmType {
    Emphasis,
    Deemphasis,
}

pub fn parse_emphasis(pair: Pair<'_, Rule>) -> Emphasis {
    let mut iter = pair.into_inner();
    let strength_type_raw = iter.next().expect("IP: parse_emphasis: no strength_type").as_str();
    let text_raw = iter.next().expect("IP: parse_emphasis: no text");
    let text = parse_string(text_raw);
    let (strength, etype) = match strength_type_raw {
        "le" => (EmStrength::Light, EmType::Emphasis),
        "me" => (EmStrength::Medium, EmType::Emphasis),
        "se" => (EmStrength::Strong, EmType::Emphasis),
        "ld" => (EmStrength::Light, EmType::Deemphasis),
        "md" => (EmStrength::Medium, EmType::Deemphasis),
        "sd" => (EmStrength::Strong, EmType::Deemphasis),
        _ => panic!("IP: parse_emphasis: wrong strength_type;")
    };
    Emphasis {
        strength,
        etype,
        text,
    }
}

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct List {
    ltype: ListType,
    items: Vec<ListItem>,
    tags: Tags,
    props: Props,
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ListType {
    Distinct,
    #[default] Identical,
    Checked,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ListItem {
    Text(String),
    MText(TextWithMeta),
    Em(Emphasis),
    Code(Result<CodeBlock, CodeIdentError>),
    List(List),
}

pub fn parse_list(pair: Pair<'_, Rule>) -> List {
    let mut items = Vec::new();
    let mut tags = Tags::default();
    let mut props = Props::default();
    let mut iter = pair.into_inner();
    let ltype = match iter.next().expect("IP: parse_list: no type;").as_str() {
        "dl" => ListType::Distinct,
        "il" => ListType::Identical,
        "cl" => ListType::Checked,
        _ => panic!("IP: parse_list: impossble list type;"),
    };
    for inner in iter {
        match inner.as_rule() {
            Rule::text_item => {
                let text = parse_text_item(inner);
                if text.meta_is_empty() {
                    items.push(ListItem::Text(text.text));
                } else {
                    items.push(ListItem::MText(text));
                }
            },
            Rule::emphasis => items.push(ListItem::Em(parse_emphasis(inner))),
            Rule::code => items.push(ListItem::Code(parse_code(inner))),
            Rule::list => items.push(ListItem::List(parse_list(inner))),
            Rule::tags => tags.absorb(parse_tags(inner)),
            Rule::props => props.absorb(parse_props(inner)),
            r => panic!("IP: parse_heading: illegal rule: {:?};", r),
        }
    }
    List {
        ltype,
        items,
        tags,
        props,
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

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum CodeModeHint {
    #[default] Show,
    Choice,
    Auto,
    Replace,
}

fn parse_code(pair: Pair<'_, Rule>) -> Result<CodeBlock, CodeIdentError> {
    let mut iter = pair.into_inner();
    let lang_raw = iter.next().expect("IP: parse_code: no language;");
    let mode_raw = iter.next().expect("IP: parse_code: no mode;");
    let code_raw = iter.next().expect("IP: parse_code: no code;");
    let (tags, props) = if let Some(next) = iter.next() {
        if let Some(tags) = parse_tags(next.clone()) {
            if let Some(next) = iter.next() {
                (tags, parse_props(next))
            } else {
                (tags, Props::default())
            }
        } else {
            (Tags::default(), parse_props(next))
        }
    } else {
        (Tags::default(), Props::default())
    };
    let language = parse_string(lang_raw);
    let mode = parse_code_mode(mode_raw);
    let code = parse_code_text(code_raw)?;
    Ok(CodeBlock {
        language,
        mode,
        code,
        tags,
        props,
    })
}

fn parse_code_mode(pair: Pair<'_, Rule>) -> CodeModeHint {
    let string = parse_string(pair);
    match string.as_ref() {
        "choice" => CodeModeHint::Choice,
        "auto" => CodeModeHint::Auto,
        "replace" => CodeModeHint::Replace,
        _ => CodeModeHint::Show,
    }
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

fn parse_string(pair: Pair<'_, Rule>) -> String {
    let inner = pair.into_inner().next().expect("IP: parse_string: no inner;");
    inner.as_str().chars().filter(|c| *c != '\n' && *c != '\r').collect()
}

fn parse_text(pair: Pair<'_, Rule>) -> String {
    parse_text_string(pair.into_inner().next().expect("IP: parse_text: no inner;").as_str())
}

fn parse_text_item(pair: Pair<'_, Rule>) -> TextWithMeta {
    let mut iter = pair.into_inner();
    let string_raw = iter.next().expect("IP: parse_text: no inner;").into_inner().as_str();
    let text = parse_text_string(string_raw);
    let (tags, props) = if let Some(next) = iter.next() {
        if let Some(tags) = parse_tags(next.clone()) {
            if let Some(next) = iter.next() {
                (tags, parse_props(next))
            } else {
                (tags, Props::default())
            }
        } else {
            (Tags::default(), parse_props(next))
        }
    } else {
        (Tags::default(), Props::default())
    };
    TextWithMeta {
        text,
        tags,
        props,
    }
}

fn parse_text_string(string: &str) -> String {
    let mut res = String::new();
    let mut last_nl = true;
    let mut last_ws = false;
    let mut fresh = true;
    for c in string.chars() {
        match c {
            '\n' => {
                if !last_nl {
                    last_nl = true;
                    res.push('\n');
                }
                fresh = false;
            },
            '\r' => {},
            x => {
                if x.is_whitespace() {
                    if !last_ws {
                        if !last_nl || fresh {
                            res.push(x);
                        }
                        last_ws = true;
                    }
                } else {
                    last_nl = false;
                    last_ws = false;
                    res.push(x);
                }
            },
        }
    }
    if let Some(last) = res.chars().last() {
        if last == '\n' {
            res.pop();
        }
    }
    res
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct CodeIdentError;

fn parse_code_text(pair: Pair<'_, Rule>) -> Result<String, CodeIdentError> {
    let mut iter = pair.into_inner();
    let start = iter.next().expect("IP: parse_text: no start;");
    let inner = iter.next().expect("IP: parse_text: no inner;");
    let (_, start_col) = start.line_col();
    let raw = inner.as_str().to_string();
    let mut res = String::new();
    let mut identc = start_col;
    let mut first_nl = true;
    for c in raw.chars() {
        match c {
            ' ' => {
                if identc < start_col - 1 {
                    identc += 1;
                } else {
                    res.push(c);
                }
            },
            '\n' => {
                identc = 0;
                if first_nl {
                    first_nl = false;
                } else {
                    res.push(c);
                }
            },
            '\r' => {},
            _ => {
                if identc < start_col - 1 {
                    return Err(CodeIdentError);
                } else {
                    res.push(c);
                }
            },
        }
    }
    if res.ends_with('\n') {
        res.pop();
    }
    Ok(res)
}

fn _parse_uint(pair: Pair<'_, Rule>) -> Result<u64, ParseIntError> {
    pair.as_str().parse()
}

fn parse_uint_capped(pair: Pair<'_, Rule>) -> u64 {
    pair.as_str().parse().expect("IP: parse_uint_capped: uint with more than 19 numbers;")
}

fn parse_int(pair: Pair<'_, Rule>) -> Result<i64, ParseIntError> {
    pair.as_str().parse()
}

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

fn parse_date(pair: Pair<'_, Rule>) -> Result<Date, DateError> {
    let mut iter = pair.as_str().split("/");
    let ys = iter.next().expect("IP: parse_date: no year;");
    let ms = iter.next().expect("IP: parse_date: no month;");
    let ds = iter.next().expect("IP: parse_date: no day;");
    Date::new(
        ys.parse().map_err(DateError::Parsing)?,
        ms.parse().map_err(DateError::Parsing)?,
        ds.parse().map_err(DateError::Parsing)?,
    )
}

