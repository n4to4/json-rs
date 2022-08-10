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
    UnexpectedToken(JsonToken),
    SyntaxError,
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

    pub fn expression(&mut self) -> Result<JsonValue> {
        match self.advance() {
            JsonToken::Eof => {}
            JsonToken::Null => {
                self.state.parsed_value = Some(JsonValue::Null);
            }
            JsonToken::Number(n) => {
                self.state.parsed_value = Some(JsonValue::Number(n));
            }
            JsonToken::String(s) => {
                self.state.parsed_value = Some(JsonValue::String(s));
            }
            // array
            JsonToken::LeftSquareBracket => {
                let arr = self.array()?;
                self.state.parsed_value = Some(JsonValue::Array(arr));
            }
            // object
            JsonToken::LeftBrace => {
                let obj = self.object()?;
                self.state.parsed_value = Some(JsonValue::Object(obj));
            }
            tok => {
                return Err(ParseError::UnexpectedToken(tok));
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

    fn matches(&mut self, one_of: Vec<JsonToken>) -> Option<JsonToken> {
        let cur = self.state.current;
        if cur >= self.tokens.len() {
            return None;
        }
        let peek = self.tokens[cur].clone();
        for tok in &one_of {
            if peek == *tok {
                self.advance();
                return Some(peek);
            }
        }
        None
    }

    fn array(&mut self) -> Result<Vec<JsonValue>> {
        let mut v = Vec::new();
        loop {
            let tok = self.advance();
            self.current_token();
            match tok {
                JsonToken::RightSquareBracket => {
                    return Ok(v);
                }
                JsonToken::Number(n) => {
                    v.push(JsonValue::Number(n));
                    match self.matches(vec![JsonToken::Comma, JsonToken::RightSquareBracket]) {
                        Some(JsonToken::Comma) => {}
                        Some(JsonToken::RightSquareBracket) => {
                            return Ok(v);
                        }
                        Some(t) => return Err(ParseError::UnexpectedToken(t)),
                        None => {
                            return Err(ParseError::SyntaxError);
                        }
                    }
                }
                JsonToken::String(s) => {
                    v.push(JsonValue::String(s));
                    match self.matches(vec![JsonToken::Comma, JsonToken::RightSquareBracket]) {
                        Some(JsonToken::Comma) => {}
                        Some(JsonToken::RightSquareBracket) => {
                            return Ok(v);
                        }
                        Some(t) => return Err(ParseError::UnexpectedToken(t)),
                        None => {
                            return Err(ParseError::SyntaxError);
                        }
                    }
                }
                JsonToken::Null => {
                    v.push(JsonValue::Null);
                    match self.matches(vec![JsonToken::Comma, JsonToken::RightSquareBracket]) {
                        Some(JsonToken::Comma) => {}
                        Some(JsonToken::RightSquareBracket) => {
                            return Ok(v);
                        }
                        Some(t) => return Err(ParseError::UnexpectedToken(t)),
                        None => {
                            return Err(ParseError::SyntaxError);
                        }
                    }
                }
                _ => return Err(ParseError::SyntaxError),
            }
        }
    }

    fn object(&mut self) -> Result<HashMap<String, JsonValue>> {
        todo!()
    }
}

pub fn parse(src: &str) -> Result<JsonValue, ParseError> {
    let scanner = Scanner::new(src.to_owned());
    let tokens = scanner.scan_tokens().map_err(ParseError::ScanError)?;
    let mut parser = Parser::new(tokens);
    parser.expression()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_array() {
        let json = parse(r#"[42]"#).unwrap();
        assert!(matches!(json, JsonValue::Array(_)));
        match json {
            JsonValue::Array(arr) => match &arr[..] {
                &[JsonValue::Number(42)] => {}
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}
