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
    props: Props,
    tags: Tags,
    errors: Vec<DocError>,
    items: Vec<DocItem>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DocError {
    Props(PropValError),
    Code(CodeError),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DocItem {
    Text(String),
    Emphasis(Emphasis),
    Code(CodeBlock),
}

pub trait Absorb {
    fn absorb(&mut self, other: Self);
}

pub fn parse(input: &str) -> Result<Doc, String> {
    let mut doc = Doc::default();
    let pairs = match IncodocParser::parse(Rule::top, input) {
        Ok(res) => res,
        Err(e) => {
            return Err(e.to_string());
        },
    };
    for inner in pairs {
        match inner.as_rule() {
            Rule::props => {
                let props = parse_props(inner);
                doc.props.absorb(props);
            },
            Rule::tags => {
                let tags = parse_tags(inner);
                doc.tags.absorb(tags);
            },
            Rule::text => {
                doc.items.push(DocItem::Text(parse_text(inner)));
            },
            Rule::emphasis => {
                doc.items.push(DocItem::Emphasis(parse_emphasis(inner)));
            },
            Rule::code => {
                match parse_code(inner) {
                    Ok(code_block) => doc.items.push(DocItem::Code(code_block)),
                    Err(err) => doc.errors.push(DocError::Code(err)),
                }
            },
            _ => {},
        }
    }
    Ok(doc)
}

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Props {
    map: HashMap<String, PropVal>,
    errors: Vec<PropValError>,
}

#[derive(Clone, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum PropVal {
    String(String),
    Text(String),
    Int(i64),
    Date(Date),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PropValError {
    Int(ParseIntError),
    Date(DateError),
}

impl Props {
    pub fn from(map: HashMap<String, PropVal>, errors: Vec<PropValError>) -> Self {
        Self {
            map,
            errors,
        }
    }
}

impl Absorb for Props {
    fn absorb(&mut self, other: Self) {
        for (k, v) in other.map {
            self.map.insert(k, v);
        }
        for err in other.errors {
            self.errors.push(err);
        }
    }
}

fn parse_props(pair: Pair<'_, Rule>) -> Props {
    let mut map = HashMap::new();
    let mut errors = Vec::new();
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::prop_tuple => {
                match parse_prop_tuple(inner) {
                    Ok((key, value)) => { map.insert(key, value); },
                    Err(err) => errors.push(err),
                }
            },
            r => {
                panic!("IP: parse_props: illegal rule: {:?};", r);
            },
        }
    }
    Props {
        map,
        errors,
    }
}

fn parse_prop_tuple(pair: Pair<'_, Rule>) -> Result<(String, PropVal), PropValError> {
    let mut inners = pair.into_inner();
    let string = inners.next().expect("IP: parse_prop_tuple: no string;");
    let prop_val = inners.next().expect("IP: parse_prop_tuple: no prop_val;");
    let string = parse_string(string);
    let prop_val = parse_prop_val(prop_val)?;
    Ok((string, prop_val))
}

fn parse_prop_val(pair: Pair<'_, Rule>) -> Result<PropVal, PropValError> {
    Ok(match pair.as_rule() {
        Rule::string => {
            PropVal::String(parse_string(pair))
        },
        Rule::text => {
            PropVal::Text(parse_text(pair))
        },
        Rule::int => {
            PropVal::Int(parse_int(pair).map_err(PropValError::Int)?)
        },
        Rule::date => {
            PropVal::Date(parse_date(pair).map_err(PropValError::Date)?)
        },
        r => {
            panic!("IP: parse_prop_val: illegal rule: {:?};", r);
        },
    })
}

pub type Tags = HashSet<String>;

impl Absorb for Tags {
    fn absorb(&mut self, other: Self) {
        for v in other {
            self.insert(v);
        }
    }
}

fn parse_tags(pair: Pair<'_, Rule>) -> Tags {
    let mut res = HashSet::new();
    for strings in pair.into_inner() {
        for string in strings.into_inner() {
            res.insert(parse_string(string));
        }
    }
    res
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
pub struct CodeBlock {
    pub language: String,
    pub mode: CodeModeHint,
    pub code: String,
    pub props: Props,
    pub tags: Tags,
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum CodeModeHint {
    #[default] Show,
    Choice,
    Auto,
    Replace,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CodeError {
    Ident(CodeIdentError),
    Prop(PropValError),
}

fn parse_code(pair: Pair<'_, Rule>) -> Result<CodeBlock, CodeError> {
    let mut iter = pair.into_inner();
    let lang_raw = iter.next().expect("IP: parse_code: no language;");
    let mode_raw = iter.next().expect("IP: parse_code: no mode;");
    let code_raw = iter.next().expect("IP: parse_code: no code;");
    let props = iter.next().map(parse_props).unwrap_or_default();
    let tags = iter.next().map(parse_tags).unwrap_or_default();
    let language = parse_string(lang_raw);
    let mode = parse_code_mode(mode_raw);
    let code = parse_code_text(code_raw).map_err(CodeError::Ident)?;
    Ok(CodeBlock {
        language,
        mode,
        code,
        props,
        tags,
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

fn parse_string(pair: Pair<'_, Rule>) -> String {
    let inner = pair.into_inner().next().expect("IP: parse_string: no inner;");
    inner.as_str().chars().filter(|c| *c != '\n' && *c != '\r').collect()
}

fn parse_text(pair: Pair<'_, Rule>) -> String {
    let mut iter = pair.into_inner();
    let inner = iter.next().expect("IP: parse_text: no inner;");
    let mut res = String::new();
    let mut last_nl = true;
    let mut last_ws = false;
    let mut fresh = true;
    for c in inner.as_str().chars() {
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
    // loop {
        if let Some(last) = res.chars().last() {
            if last == '\n' {
                res.pop();
            }
            // else {
            //     break;
            // }
        }
    // }
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

