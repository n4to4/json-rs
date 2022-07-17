#[derive(Debug, PartialEq, Eq)]
pub(crate) enum JsonToken {
    LBrace,
    RBrace,
    //Number(f64),
    String(String),
    // Array
    // Object
}

#[derive(Debug)]
pub(crate) struct Scanner {
    tokens: Vec<JsonToken>,
    source: String,
    bytes: Vec<u8>,
    current: i64,
}

impl Scanner {
    pub(crate) fn new(source: String) -> Self {
        let bytes = source.as_bytes().to_owned();
        Scanner {
            tokens: Vec::new(),
            source,
            bytes,
            current: -1,
        }
    }

    pub(crate) fn scan_tokens(self) -> Vec<JsonToken> {
        self.tokens
    }

    fn scan_token(&mut self) -> anyhow::Result<()> {
        match self.advance() {
            Some(b'"') => self.string()?,
            Some(_) => {}
            None => {}
        }
        Ok(())
    }

    fn advance(&mut self) -> Option<u8> {
        if self.is_at_end() {
            None
        } else {
            self.current += 1;
            Some(self.bytes[self.current as usize])
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_string() {
        let json = r#""foo""#;
        let mut scanner = Scanner::new(json.into());
        scanner.scan_token().unwrap();
        let tokens = scanner.scan_tokens();

        assert_eq!(&tokens, &[JsonToken::String(String::from("foo"))]);
    }
}
