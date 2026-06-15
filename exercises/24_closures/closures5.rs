// closures5.rs
//
// A function can also *return* a closure. Since each closure has an unnameable
// type, you describe the return type with `impl Fn(...) -> ...` (one concrete,
// hidden type). When you need to return different closures from different
// branches, box it instead: `Box<dyn Fn(...) -> ...>`. The returned closure
// here uses `move` so it owns `x` and keeps working after `make_adder` returns.
// (TRPL ch13.1)
//
// TODO: Fix the return type of `make_adder` so it can return the closure.
//
// Run with `rustlings run closures5`, get a hint with `rustlings hint closures5`.

// TODO: Replace `???` with the correct return type (`impl Fn(i32) -> i32`).
fn make_adder(x: i32) -> ??? {
    move |n| n + x
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_with_returned_closure() {
        let add_three = make_adder(3);
        assert_eq!(add_three(4), 7);
        assert_eq!(add_three(10), 13);

        // A freshly built adder captures a different `x`.
        assert_eq!(make_adder(100)(1), 101);
    }
}
