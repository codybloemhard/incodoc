mod tests;

use std::num::ParseIntError;

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
    meta: Meta,
    errors: Vec<MetaValError>,
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
            Rule::meta => {
                let (m, e) = parse_meta(inner);
                doc.meta = m;
                doc.errors = e;
            },
            _ => {},
        }
    }
    Ok(doc)
}

pub type Meta = Vec<MetaTuple>;

fn parse_meta(pair: Pair<'_, Rule>) -> (Meta, Vec<MetaValError>) {
    let mut res = Vec::new();
    let mut errs = Vec::new();
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::meta_tuple => {
                match parse_meta_tuple(inner) {
                    Ok(tuple) => res.push(tuple),
                    Err(err) => errs.push(err),
                }
            },
            r => {
                panic!("IP: parse_meta: illegal rule: {:?};", r);
            },
        }
    }
    (res, errs)
}

pub type MetaTuple = (String, MetaVal);

fn parse_meta_tuple(pair: Pair<'_, Rule>) -> Result<MetaTuple, MetaValError> {
    let mut inners = pair.into_inner();
    let string = inners.next().expect("IP: parse_meta_tuple: no string;");
    let meta_val = inners.next().expect("IP: parse_meta_tuple: no meta_val;");
    let string = parse_string(string);
    let meta_val = parse_meta_val(meta_val)?;
    Ok((string, meta_val))
}

#[derive(Clone, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum MetaVal {
    String(String),
    Int(i64),
    Date(Date),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MetaValError {
    Int(ParseIntError),
    Date(DateError),
}

fn parse_meta_val(pair: Pair<'_, Rule>) -> Result<MetaVal, MetaValError> {
    Ok(match pair.as_rule() {
        Rule::string => {
            MetaVal::String(parse_string(pair))
        },
        Rule::int => {
            MetaVal::Int(parse_int(pair).map_err(MetaValError::Int)?)
        },
        Rule::date => {
            MetaVal::Date(parse_date(pair).map_err(MetaValError::Date)?)
        },
        r => {
            panic!("IP: parse_meta_val: illegal rule: {:?};", r);
        },
    })
}

fn parse_string(pair: Pair<'_, Rule>) -> String {
    let inner = pair.into_inner().next().expect("IP: parse_string: no inner;");
    inner.as_str().to_string()
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

