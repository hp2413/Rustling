// errors7.rs
//
// Solution: implement `From<ParseIntError> for ParseError` so the `?` operator
// can convert a parse error into our custom error enum automatically. Because
// `ParseError` also implements `Error`, it can be boxed into `Box<dyn Error>`,
// which is how `double_all` chains `?` over many inputs.

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(PartialEq, Debug)]
enum ParseError {
    NotANumber(ParseIntError),
    Negative,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::NotANumber(e) => write!(f, "not a number: {e}"),
            ParseError::Negative => write!(f, "number must not be negative"),
        }
    }
}

impl Error for ParseError {}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> Self {
        ParseError::NotANumber(err)
    }
}

fn parse_and_double(s: &str) -> Result<i64, ParseError> {
    let n: i64 = s.parse()?;
    if n < 0 {
        return Err(ParseError::Negative);
    }
    Ok(n * 2)
}

fn double_all(inputs: &[&str]) -> Result<Vec<i64>, Box<dyn Error>> {
    let mut doubled = Vec::new();
    for &s in inputs {
        doubled.push(parse_and_double(s)?);
    }
    Ok(doubled)
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doubles_a_number() {
        assert_eq!(parse_and_double("21"), Ok(42));
    }

    #[test]
    fn parse_error_is_converted() {
        assert!(matches!(
            parse_and_double("oops"),
            Err(ParseError::NotANumber(_)),
        ));
    }

    #[test]
    fn rejects_negative() {
        assert_eq!(parse_and_double("-1"), Err(ParseError::Negative));
    }

    #[test]
    fn boxed_errors_chain() {
        assert_eq!(double_all(&["1", "2", "3"]).unwrap(), vec![2, 4, 6]);
        assert!(double_all(&["1", "nope", "3"]).is_err());
    }
}
