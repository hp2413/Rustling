// errors7.rs
//
// Real programs hit *several* kinds of error. A clean way to handle them is to
// define your own error `enum` with one variant per failure mode, then teach the
// `?` operator how to convert foreign errors into it by implementing
// `From<TheirError> for YourError`. Once that `From` exists, `?` will silently
// convert the foreign error and return early. Implementing `std::error::Error`
// (which requires `Display` + `Debug`) additionally lets your error be boxed into
// a `Box<dyn Error>` — the catch-all "any error" type that `?` can also target.
// (TRPL ch9.2; RBE "Error handling")
//
// TODO: Implement `From<ParseIntError> for ParseError` (see the commented-out
// block below) so that the `?` after `.parse()` in `parse_and_double` compiles by
// converting the parse error into our own `ParseError::NotANumber` variant.
//
// Run with `rustlings run errors7`, get a hint with `rustlings hint errors7`.

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(PartialEq, Debug)]
enum ParseError {
    NotANumber(ParseIntError),
    Negative,
}

// Implementing `Display` (plus the derived `Debug`) lets us implement `Error`,
// which in turn lets a `ParseError` be turned into a `Box<dyn Error>` below.
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::NotANumber(e) => write!(f, "not a number: {e}"),
            ParseError::Negative => write!(f, "number must not be negative"),
        }
    }
}

impl Error for ParseError {}

// TODO: Implement this so `?` can convert a `ParseIntError` into a `ParseError`.
// impl From<ParseIntError> for ParseError {
//     fn from(err: ParseIntError) -> Self {
//         ???
//     }
// }

fn parse_and_double(s: &str) -> Result<i64, ParseError> {
    // The `?` below needs `From<ParseIntError> for ParseError` to exist so it can
    // convert the `ParseIntError` from `parse()` into our `ParseError`.
    let n: i64 = s.parse()?;
    if n < 0 {
        return Err(ParseError::Negative);
    }
    Ok(n * 2)
}

// `Box<dyn Error>` accepts ANY type that implements `Error`, so the `?` here can
// chain on the `ParseError` returned by `parse_and_double` without us writing a
// `From` impl for the box (the standard library already provides one).
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
        // A non-numeric input becomes our `NotANumber` variant via the `From` impl.
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
        // A bad value short-circuits the whole thing through `?`.
        assert!(double_all(&["1", "nope", "3"]).is_err());
    }
}
