#![allow(dead_code)]

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum JsonToken {
    LeftBrace,
    RightBrace,
    LeftSquareBracket,
    RightSquareBracket,
    Colon,
    Comma,

    Number(u64),
    String(String),
    Null,

    Eof,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum ScanError {
    #[error("syntax error")]
    SyntaxError,
}

#[derive(Debug)]
pub(crate) struct Scanner {
    tokens: Vec<JsonToken>,
    source: String,
    bytes: Vec<u8>,
    start: i64,
    current: i64,
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

    pub(crate) fn scan_tokens(mut self) -> anyhow::Result<Vec<JsonToken>> {
        if !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }
        Ok(self.tokens)
    }

    fn scan_token(&mut self) -> anyhow::Result<()> {
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
                    } else {
                        return Err(ScanError::SyntaxError.into());
                    }
                }
            }
        } else {
            self.tokens.push(JsonToken::Eof);
        }
        Ok(())
    }

    fn advance(&mut self) -> Option<u8> {
        if self.is_at_end() {
            None
        } else {
            let cur = self.current;
            self.current += 1;
            Some(self.bytes[cur as usize])
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.bytes.len() as i64
    }

    fn string(&mut self) -> anyhow::Result<()> {
        let mut bytes = Vec::new();
        while let Some(c) = self.advance() {
            if c == b'"' {
                break;
            }
            bytes.push(c);
        }
        let s = String::from_utf8(bytes)?;
        self.tokens.push(JsonToken::String(s));
        Ok(())
    }

    fn number(&mut self) -> anyhow::Result<()> {
        while let Some(c) = self.advance() {
            if !c.is_ascii_digit() {
                break;
            }
        }
        let s = &self.source[self.start as usize..self.current as usize];
        self.tokens.push(JsonToken::Number(s.parse()?));
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
        assert_eq!(&tokens, &[JsonToken::String(String::from("foo"))]);
    }

    #[test]
    fn test_scan_number() {
        let json = r#"42"#;
        let scanner = Scanner::new(json.into());
        let tokens = scanner.scan_tokens().expect("should be successful");
        assert_eq!(&tokens, &[JsonToken::Number(42)]);
    }
}
