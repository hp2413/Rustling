// closures5.rs
//
// Solution: describe the returned closure's type with `impl Fn(i32) -> i32`.
// The `move` keyword lets the closure own `x` so it stays valid after the
// function returns.

fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
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
