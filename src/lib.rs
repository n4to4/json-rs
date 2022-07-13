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

pub fn parse(src: &str) -> Result<JsonValue, ParseError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple() {
        let actual = parse("42").unwrap();
        assert!(matches!(actual, JsonValue::Number(n) if n == 42.0));
    }
}
