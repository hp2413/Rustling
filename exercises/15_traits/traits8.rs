// traits8.rs
//
// Operators in Rust are syntactic sugar for trait methods: writing `a + b`
// actually calls `a.add(b)` from the `std::ops::Add` trait. So you can make `+`
// work on your OWN types simply by implementing `Add` for them. The trait has
// an associated type `Output` that says what `+` produces. (TRPL ch19.3
// "Operator Overloading")
//
// TODO: Implement `Add` for `Point` so that `p1 + p2` adds the `x` and `y`
// fields component-wise and returns a new `Point`.
//
// Run with `rustlings run traits8`, get a hint with `rustlings hint traits8`.

use std::ops::Add;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

// TODO: Fill in this `impl` block. You need to:
//   1. set the associated type:   `type Output = Point;`
//   2. write the method:          `fn add(self, other: Point) -> Point { ... }`
//      returning a `Point` whose fields are the sums of `self`'s and `other`'s.
impl Add for Point {
    ???
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_component_wise() {
        let a = Point { x: 1, y: 2 };
        let b = Point { x: 3, y: 4 };
        assert_eq!(a + b, Point { x: 4, y: 6 });
    }

    #[test]
    fn adds_to_origin() {
        assert_eq!(
            Point { x: -5, y: 10 } + Point { x: 5, y: -10 },
            Point { x: 0, y: 0 },
        );
    }
}
