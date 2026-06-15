// patterns1.rs
//
// A `match` arm can be refined in two ways used here:
//   * a *guard* — an extra `if` condition after the pattern, e.g.
//     `n if n < 0 => ...`, which only takes the arm when the condition holds;
//   * a *range* pattern — `1..=9` matches any value from 1 to 9 inclusive.
// `match` must be exhaustive, so a final `_` arm catches everything else.
// (TRPL "Patterns and Matching"; RBE "Flow of Control".)
//
// TODO: Fill in the two missing arm patterns so `classify` sorts an integer
// into "negative", "zero", "single digit" (1..=9), or "big".
//
// Run with `rustlings run patterns1`, get a hint with `rustlings hint patterns1`.

fn classify(n: i32) -> &'static str {
    match n {
        // TODO: Match negative numbers using an `if` guard (e.g. `n if n < 0`).
        ??? => "negative",
        0 => "zero",
        // TODO: Match the single digits using the inclusive range `1..=9`.
        ??? => "single digit",
        _ => "big",
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_numbers() {
        assert_eq!(classify(-5), "negative");
        assert_eq!(classify(0), "zero");
        assert_eq!(classify(1), "single digit");
        assert_eq!(classify(9), "single digit");
        assert_eq!(classify(10), "big");
        assert_eq!(classify(1000), "big");
    }
}
