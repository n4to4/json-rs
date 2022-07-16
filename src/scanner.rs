#[derive(Debug)]
pub(crate) enum JsonToken {
    LBrace,
    RBrace,
    Number(f64),
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

    fn scan_token(&mut self) {
        let c = self.advance();
    }

    fn advance(&mut self) -> u8 {
        self.current += 1;
        self.bytes[self.current as usize]
    }
}
