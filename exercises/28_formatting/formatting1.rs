// formatting1.rs
//
// Rust's `println!`, `print!`, `format!` and friends use a mini-language inside
// the `{}` placeholders to control *how* a value is rendered:
//   - positional args:   `{0}` / `{1}` pick arguments by index
//   - named args:        `{name}` uses an argument passed as `name = ...`
//   - precision:         `{:.2}` keeps 2 digits after the decimal point
//   - width + alignment: `{:>8}` right-aligns within a field 8 characters wide
//   - debug:             `{:?}` prints the `Debug` representation
// `format!` builds a `String` instead of printing — handy for tests.
// (RBE "Formatted print"; TRPL ch5.2)
//
// TODO: Replace each `???` below with a format string literal that makes the
// assertion pass. Only the format string changes — leave the arguments as they
// are.
//
// Run with `rustlings run formatting1`, get a hint with `rustlings hint formatting1`.

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn positional_arguments() {
        // The two arguments are "world" then "hello"; use positional `{1}` and
        // `{0}` to swap their order in the output.
        let s = format!(???, "world", "hello");
        assert_eq!(s, "hello world");
    }

    #[test]
    fn named_arguments() {
        // Refer to the argument by the name it was given (`name`).
        let s = format!(???, name = "Ferris");
        assert_eq!(s, "Hi, Ferris!");
    }

    #[test]
    fn precision() {
        let value = 1.23456;
        // Print `value` with exactly two digits after the decimal point.
        let s = format!(???, value);
        assert_eq!(s, "1.23");
    }

    #[test]
    fn width_and_alignment() {
        // Right-align the number inside a field that is 8 characters wide,
        // padding the left with spaces.
        let s = format!(???, 42);
        assert_eq!(s, "      42");
    }

    #[test]
    fn debug_output() {
        let v = vec![1, 2, 3];
        // A `Vec` has no `Display` (`{}`) form, but it does have a `Debug`
        // (`{:?}`) one.
        let s = format!(???, v);
        assert_eq!(s, "[1, 2, 3]");
    }
}
