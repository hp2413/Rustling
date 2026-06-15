// closures4.rs
//
// Functions can take closures as parameters — this is how higher-order
// functions are written in Rust. Every closure has its own unique, unnameable
// type, so you accept one with a *generic* type parameter constrained by a
// closure trait: `Fn`, `FnMut`, or `FnOnce`. A closure that only reads its
// captures (or captures nothing) implements `Fn`. (TRPL ch13.1)
//
// TODO: Complete the trait bound on `F` so `apply` accepts any closure that
// takes an `i32` and returns an `i32`.
//
// Run with `rustlings run closures4`, get a hint with `rustlings hint closures4`.

// TODO: Replace `???` with the trait bound for a closure `i32 -> i32`.
fn apply<F: ???>(f: F, x: i32) -> i32 {
    f(x)
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn applies_closure() {
        assert_eq!(apply(|n| n * 2, 5), 10);
        assert_eq!(apply(|n| n + 100, 1), 101);
    }
}
