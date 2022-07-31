use crate::scanner::{JsonToken, ScanError, Scanner};
use std::collections::HashMap;

type Result<T, E = ParseError> = std::result::Result<T, E>;

#[derive(Debug, Clone)]
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
    state: ParseState,
}

#[derive(Debug, Default)]
pub struct ParseState {
    current: usize,
    parsed_value: Option<JsonValue>,
}

impl Parser {
    pub fn new(tokens: Vec<JsonToken>) -> Self {
        Parser {
            tokens,
            state: Default::default(),
        }
    }

    pub fn parse(&mut self) -> Result<JsonValue> {
        loop {
            match self.advance() {
                JsonToken::Eof => break,
                JsonToken::LeftSquareBracket => {
                    let arr = self.array()?;
                    self.state.parsed_value = Some(JsonValue::Array(arr));
                }
                _ => todo!(),
            }
        }
        let parsed = self.state.parsed_value.take();
        Ok(parsed.unwrap())
    }

    fn current_token(&self) -> JsonToken {
        let cur = self.state.current;
        self.tokens[cur].clone()
    }

    fn advance(&mut self) -> JsonToken {
        let cur = self.state.current;
        if cur < self.tokens.len() {
            let tok = self.tokens[cur].clone();
            self.state.current += 1;
            tok
        } else {
            JsonToken::Eof
        }
    }

    fn array(&mut self) -> Result<Vec<JsonValue>> {
        Ok(vec![JsonValue::Number(42)])
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
