// derive1.rs
//
// Many common traits are so mechanical that the compiler can write them for you
// with the `#[derive(...)]` attribute — no hand-written `impl` needed. Four of
// the most common:
//
//   * `Debug`     — enables the `{:?}` formatter (and `{:#?}` for pretty print).
//   * `Clone`     — gives an explicit `.clone()` that duplicates the value.
//   * `Copy`      — makes the type copy implicitly on assignment / pass-by-value
//                   instead of MOVING (only allowed when every field is `Copy`).
//                   `Copy` requires `Clone`, so you derive them together.
//   * `PartialEq` — lets you compare values with `==` and `!=`.
//
// TODO: Add a single `#[derive(...)]` line above `Point` so this file compiles
// and the tests pass. Work out which traits are needed from how the tests use a
// `Point`: it is printed with `{:?}`, used again after being copied, duplicated
// with `.clone()`, and compared with `==`.
//
// Run with `rustlings run derive1`, get a hint with `rustlings hint derive1`.

// TODO: Add the derive attribute here.
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_lets_value_be_used_after_assignment() {
        let a = Point { x: 1, y: 2 };
        // Without `Copy`, `let b = a;` would MOVE `a`, and using `a` afterwards
        // would fail to compile. With `Copy`, `b` is an independent copy.
        let b = a;
        assert_eq!(a.x, 1);
        assert_eq!(b.y, 2);
    }

    #[test]
    fn clone_duplicates_a_collection() {
        let points = vec![Point { x: 3, y: 4 }, Point { x: 5, y: 6 }];
        // Cloning a `Vec<Point>` requires each `Point` to be `Clone`.
        let copy = points.clone();
        assert_eq!(copy, points);
    }

    #[test]
    fn partial_eq_enables_comparison() {
        let a = Point { x: 1, y: 2 };
        let b = Point { x: 1, y: 2 };
        let c = Point { x: 9, y: 9 };
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn debug_enables_formatting() {
        let a = Point { x: 1, y: 2 };
        // The `{:?}` formatter needs `Debug`.
        assert_eq!(format!("{a:?}"), "Point { x: 1, y: 2 }");
    }
}
