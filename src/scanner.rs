#[derive(Debug)]
enum JsonToken {
    LBrace,
    RBrace,
    Number(f64),
    String(String),
    // Array
    // Object
}

pub(crate) struct Scanner {
    source: String,
    values: Vec<JsonToken>,
}

impl Scanner {
    pub(crate) fn new(source: String) -> Self {
        Scanner {
            source,
            values: Vec::new(),
        }
    }
}
