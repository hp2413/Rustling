// tests4.rs
//
// Solution: doc-tests are the ```rust blocks inside `///` comments; `cargo test`
// compiles and runs them (for library crates). The visible example calls
// `add_two` and asserts the result; the hidden `#` line provides a definition so
// the example is self-contained. We also fix the `add_two` bug so the ordinary
// unit test passes under Rustlings' binary build.

/// Adds two to `n`.
///
/// # Examples
///
/// ```
/// # fn add_two(n: i32) -> i32 { n + 2 }
/// assert_eq!(add_two(2), 4);
/// ```
pub fn add_two(n: i32) -> i32 {
    n + 2
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two() {
        assert_eq!(add_two(2), 4);
        assert_eq!(add_two(40), 42);
    }
}
