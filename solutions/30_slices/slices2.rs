// slices2.rs
//
// Solution: scan the bytes for the first space and return the slice up to it;
// if there is no space, the whole string is a single word.

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];
        }
    }

    s
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
