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
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<JsonToken>,
}

impl Parser {
    pub fn new(tokens: Vec<JsonToken>) -> Self {
        Parser { tokens }
    }

    pub fn parse(self) -> Result<JsonValue, ParseError> {
        todo!()
    }
}

pub fn parse(src: String) -> Result<JsonValue, ParseError> {
    let scanner = Scanner::new(src);
    let tokens = scanner.scan_tokens().map_err(ParseError::ScanError)?;
    let parser = Parser::new(tokens);
    parser.parse()
}
