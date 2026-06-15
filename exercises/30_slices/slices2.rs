// slices2.rs
//
// A string slice (`&str`) is a borrowed view into part of a `String` (or a
// string literal), written `&s[0..i]`. The classic example is `first_word`,
// which returns the first word of a string *as a slice of the original* — no
// new allocation, just a borrow that points into the input. To find where the
// first word ends, scan the bytes (`s.as_bytes()`) for a space (`b' '`).
// (TRPL ch4.3 "String Slices")
//
// TODO: Implement `first_word` so it returns everything up to the first space,
// or the whole string when there is no space.
//
// Run with `rustlings run slices2`, get a hint with `rustlings hint slices2`.

fn first_word(s: &str) -> &str {
    // TODO: Iterate over `s.as_bytes()` (using `.iter().enumerate()`); when you
    // hit a space byte (`b' '`) at index `i`, return `&s[0..i]`. If the loop
    // finishes without finding a space, the whole string is one word — return
    // `s`. Replace this `???` with that logic.
    ???
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_word_of_a_sentence() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("foo bar baz"), "foo");
    }

    #[test]
    fn no_spaces_is_one_word() {
        assert_eq!(first_word("single"), "single");
        assert_eq!(first_word(""), "");
    }
}
