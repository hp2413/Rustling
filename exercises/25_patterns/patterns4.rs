// patterns4.rs
//
// When you only care about ONE pattern, a full `match` is overkill. Rust has
// three ergonomic single-pattern forms:
//   * `if let Some(n) = opt { ... } else { ... }` — run code only when the
//     pattern matches;
//   * `let Ok(n) = expr else { return ...; };` — bind on success, or take an
//     early-exit path in the `else` block (which MUST diverge);
//   * `while let Some(x) = it.pop() { ... }` — keep looping while the pattern
//     matches, e.g. to drain a collection.
// (TRPL "Pattern syntax"; RBE "Flow of Control".)
//
// TODO: Implement all three functions using the form named in each comment.
//
// Run with `rustlings run patterns4`, get a hint with `rustlings hint patterns4`.

fn double_or_zero(opt: Option<i32>) -> i32 {
    // TODO: Use `if let Some(n) = opt { n * 2 } else { 0 }`.
    ???
}

fn parse_plus_one(s: &str) -> i32 {
    // TODO: Use a `let ... else` to bind the parsed number, or return -1 from
    // the `else` block when parsing fails, then return `n + 1`.
    ???
}

fn drain_sum(mut stack: Vec<i32>) -> i32 {
    let mut total = 0;
    // TODO: Use `while let Some(top) = stack.pop()` to pop every element,
    // adding each one to `total`.
    ???
    total
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn if_let_doubles_or_zero() {
        assert_eq!(double_or_zero(Some(21)), 42);
        assert_eq!(double_or_zero(None), 0);
    }

    #[test]
    fn let_else_parses_or_exits() {
        assert_eq!(parse_plus_one("41"), 42);
        assert_eq!(parse_plus_one("not a number"), -1);
    }

    #[test]
    fn while_let_drains_stack() {
        assert_eq!(drain_sum(vec![1, 2, 3, 4]), 10);
        assert_eq!(drain_sum(vec![]), 0);
    }
}
