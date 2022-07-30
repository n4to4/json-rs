use crate::scanner::{JsonToken, ScanError, Scanner};
use std::collections::HashMap;

#[derive(Debug)]
pub enum JsonValue {
    Null,
    Number(u64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

#[derive(Debug)]
pub enum ParseError {
    ScanError(ScanError),
    Invalid,
    EmptyInput,
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<JsonToken>,
}

#[derive(Debug)]
pub struct ParsedResult {
    value: JsonValue,
}

impl Parser {
    pub fn new(tokens: Vec<JsonToken>) -> Self {
        Parser { tokens }
    }

    pub fn parse(&mut self) -> Result<JsonValue, ParseError> {
        let mut i = 0;
        match self.tokens[i] {
            JsonToken::LeftSquareBracket => todo!(),
            _ => todo!(),
        }
    }
}

pub fn parse(src: &str) -> Result<JsonValue, ParseError> {
    let scanner = Scanner::new(src.to_owned());
    let tokens = scanner.scan_tokens().map_err(ParseError::ScanError)?;
    let mut parser = Parser::new(tokens);
    parser.parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let json = parse(r#"[42]"#).expect("parse failure");
        assert!(matches!(json, JsonValue::Array(_)));
        match json {
            JsonValue::Array(arr) => match &arr[..] {
                &[JsonValue::Number(42)] => {}
                _ => panic!("must be a vec"),
            },
            _ => panic!("must be an array"),
        }
    }
}
