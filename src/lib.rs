use pest::{
    Parser,
    // iterators::Pair,
};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parse/incodoc.pest"]
pub struct IncodocParser;

pub fn parse(input: &str) -> bool {
    let _snippet = match IncodocParser::parse(Rule::top, input) {
        Ok(res) => res,
        Err(e) => {
            println!("failed to parse: {:?}", e);
            return false;
        },
    };
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(parse(""), true);
        assert_eq!(parse("oof"), false);
    }
}
