use crate::scanner::Scanner;
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
    Invalid,
}

#[derive(Debug)]
pub struct Parser {
    scanner: Scanner,
}

impl Parser {
    pub fn new(src: String) -> Self {
        Parser {
            scanner: Scanner::new(src),
        }
    }

    pub fn parse() -> Result<JsonValue, ParseError> {
        todo!()
    }
}
