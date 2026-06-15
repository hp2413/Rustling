// closures2.rs
//
// By default a closure captures the variables it uses by the *least*
// restrictive means it can: by reference when a reference is enough. That means
// the captured variable is only *borrowed*, so you can keep using it after the
// closure. Adding the `move` keyword forces the closure to take *ownership* of
// what it captures (handy when the closure must outlive the current scope, e.g.
// when moved into a thread), but then the original binding can no longer be
// used. (NOTES "move closures"; TRPL ch13.1)
//
// TODO: Make `sum_all` a closure that borrows `numbers` and returns the sum of
// its elements, so that `numbers` is still usable on the line that follows.
//
// Run with `rustlings run closures2`, get a hint with `rustlings hint closures2`.

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn closure_borrows_by_default() {
        let numbers: Vec<i32> = (1..=4).collect();

        // TODO: Replace `???` with a closure (no `move`) that returns the sum
        // of `numbers`. Because a closure borrows by default, `numbers` is
        // still usable below.
        let sum_all = ???;

        assert_eq!(sum_all(), 10);

        // This line only compiles if the closure *borrowed* `numbers` instead
        // of moving it. If you added `move`, `numbers` would be gone here.
        assert_eq!(numbers.len(), 4);
    }
}
