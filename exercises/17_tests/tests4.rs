// tests4.rs
//
// Documentation comments written with `///` aren't only for humans: every
// ```rust code block inside them is compiled and executed by `cargo test` as a
// *doc-test*. This keeps the examples in your documentation from going stale — if
// an example stops compiling, or an `assert_eq!` inside it fails, your test suite
// fails too. (RBE "Documentation"; TRPL ch14.2)
//
// A caveat for this exercise: `cargo test` only runs doc-tests for *library*
// crates, and Rustlings builds each exercise as a *binary*. So the doc-test you
// write below is graded indirectly through the ordinary unit test at the bottom.
// To watch a doc-test run for real, put the function in a library's `lib.rs` and
// run `cargo test --doc`.
//
// TODO #1: `add_two` has a bug — it subtracts instead of adding. Fix its body so
//          the unit test passes (this is what Rustlings actually checks).
// TODO #2: Finish the doc-test example in the `///` comment by writing the
//          `assert_eq!` that checks `add_two(2)` equals `4`.
//
// Run with `rustlings run tests4`, get a hint with `rustlings hint tests4`.

/// Adds two to `n`.
///
/// # Examples
///
/// ```
/// # // A doc-test is its own tiny crate, so it can't see this file's items.
/// # // We redefine `add_two` here (hidden from the rendered docs with `#`) just
/// # // to keep the example self-contained; in a library you'd instead write
/// # // `use my_crate::add_two;`.
/// # fn add_two(n: i32) -> i32 { n + 2 }
/// // TODO: assert that `add_two(2)` equals `4`.
/// ```
pub fn add_two(n: i32) -> i32 {
    n - 2 // TODO: this should add two, not subtract.
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
