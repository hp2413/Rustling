// formatting3.rs
//
// The `{}` ("Display") placeholder is for user-facing output, and unlike
// `Debug` it is never derived — you implement it yourself so you can choose
// exactly how the value should read. Implementing `fmt::Display` also gives you
// the `.to_string()` method for free.
//
// You implement it by writing `impl fmt::Display for YourType` with a single
// method `fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result`, and writing
// into `f` with the `write!` macro (just like `format!`, but targeting `f`).
// (RBE "Display"; std::fmt)
//
// TODO: Fill in the body of `fmt` so a `Point { x, y }` displays as `(x, y)`,
// e.g. `Point { x: 3, y: 4 }` prints as `(3, 4)`.
//
// Run with `rustlings run formatting3`, get a hint with `rustlings hint formatting3`.

use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Use `write!(f, ...)` to write `(x, y)` into the formatter.
        ???
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_point() {
        let p = Point { x: 3, y: 4 };
        assert_eq!(format!("{p}"), "(3, 4)");
    }

    #[test]
    fn to_string_comes_for_free() {
        let p = Point { x: -1, y: 0 };
        assert_eq!(p.to_string(), "(-1, 0)");
    }
}
