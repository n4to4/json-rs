#[derive(Debug, PartialEq, Eq, Clone)]
pub enum JsonToken {
    LeftBrace,          // {
    RightBrace,         // }
    LeftSquareBracket,  // [
    RightSquareBracket, // ]
    Colon,              // :
    Comma,              // ,

    Number(u64),
    String(String),
    Null,

    Eof,
}

#[derive(Debug, thiserror::Error)]
pub enum ScanError {
    #[error("syntax error")]
    SyntaxError,
    #[error("unexpected EOF")]
    UnexpectedEof,
    #[error("cannot parse number")]
    Number,
}

#[derive(Debug)]
pub(crate) struct Scanner {
    tokens: Vec<JsonToken>,
    source: String,
    bytes: Vec<u8>,
    start: usize,
    current: usize,
}

impl Scanner {
    pub(crate) fn new(source: String) -> Self {
        let bytes = source.as_bytes().to_owned();
        Scanner {
            tokens: Vec::new(),
            source,
            bytes,
            start: 0,
            current: 0,
        }
    }

    pub(crate) fn scan_tokens(mut self) -> Result<Vec<JsonToken>, ScanError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }
        self.tokens.push(JsonToken::Eof);
        Ok(self.tokens)
    }

    fn scan_token(&mut self) -> Result<(), ScanError> {
        if let Some(c) = self.advance() {
            match c {
                b'"' => self.string()?,
                b'{' => self.tokens.push(JsonToken::LeftBrace),
                b'}' => self.tokens.push(JsonToken::RightBrace),
                b'[' => self.tokens.push(JsonToken::LeftSquareBracket),
                b']' => self.tokens.push(JsonToken::RightSquareBracket),
                b':' => self.tokens.push(JsonToken::Colon),
                b',' => self.tokens.push(JsonToken::Comma),
                b => {
                    if b.is_ascii_digit() {
                        self.number()?;
                    } else if b.is_ascii_whitespace() {
                        // skip
                    } else if b == b'n' {
                        if self.peekn(3) == Some(b"ull") {
                            self.advance();
                            self.advance();
                            self.advance();
                            self.tokens.push(JsonToken::Null);
                        } else {
                            return Err(ScanError::SyntaxError);
                        }
                    } else {
                        return Err(ScanError::SyntaxError);
                    }
                }
            }
        }
        Ok(())
    }

    fn peek(&self) -> Option<u8> {
        if self.is_at_end() {
            None
        } else {
            let cur = self.current;
            Some(self.bytes[cur])
        }
    }

    fn peekn(&self, n: usize) -> Option<&[u8]> {
        let cur = self.current;
        let len = self.bytes.len();
        if cur + n <= len {
            Some(&self.bytes[cur..cur + n])
        } else {
            None
        }
    }

    fn advance(&mut self) -> Option<u8> {
        if self.is_at_end() {
            None
        } else {
            let cur = self.current;
            self.current += 1;
            Some(self.bytes[cur])
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.bytes.len()
    }

    fn string(&mut self) -> Result<(), ScanError> {
        while let Some(c) = self.advance() {
            if c == b'"' {
                let s = &self.source[self.start + 1..self.current - 1];
                self.tokens.push(JsonToken::String(s.to_owned()));
                return Ok(());
            }
        }
        Err(ScanError::UnexpectedEof)
    }

    fn number(&mut self) -> Result<(), ScanError> {
        while let Some(c) = self.peek() {
            if !c.is_ascii_digit() {
                break;
            }
            self.advance();
        }
        let s = &self.source[self.start..self.current];
        let n = s.parse().map_err(|_| ScanError::Number)?;
        self.tokens.push(JsonToken::Number(n));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_string() {
        let json = r#""foo""#;
        let scanner = Scanner::new(json.into());
        let tokens = scanner.scan_tokens().expect("should be successful");

        assert_eq!(
            &tokens,
            &[JsonToken::String(String::from("foo")), JsonToken::Eof]
        );
    }

    #[test]
    fn test_scan_string_error() {
        let json = r#""foo"#;
        let scanner = Scanner::new(json.into());
        let ret = scanner.scan_tokens();

        assert!(matches!(ret, Err(ScanError::UnexpectedEof)));
    }

    #[test]
    fn test_scan_number1() {
        let json = r#"42"#;
        let scanner = Scanner::new(json.into());
        let tokens = scanner.scan_tokens().unwrap();

        assert_eq!(&tokens, &[JsonToken::Number(42), JsonToken::Eof]);
    }

    #[test]
    fn test_scan_number2() {
        let json = r#"42,"#;
        let scanner = Scanner::new(json.into());
        let tokens = scanner.scan_tokens().unwrap();

        assert_eq!(
            &tokens,
            &[JsonToken::Number(42), JsonToken::Comma, JsonToken::Eof]
        );
    }
}
