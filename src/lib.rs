#![deny(unreachable_pub, private_in_public)]

mod scanner;

type Result<T, E = ParseError> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum JsonValue {
    Null,
    Number(f64),
    String(String),
    // Array
    // Object
}

#[derive(Debug)]
pub enum ParseError {
    Invalid,
}

pub fn parse(src: &str) -> Result<JsonValue> {
    todo!()
}

fn number(src: &str) -> Result<JsonValue> {
    todo!()
}
