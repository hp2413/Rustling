// formatting2.rs
//
// The `{:?}` ("Debug") placeholder prints a value in a programmer-facing form,
// but it only works for types that implement the `Debug` trait. For your own
// structs and enums you usually don't write that by hand — you ask the compiler
// to generate it with `#[derive(Debug)]`. The `{:#?}` ("pretty Debug") variant
// prints the same data spread over multiple indented lines.
// (TRPL ch5.2 "Adding Useful Functionality with Derived Traits")
//
// TODO: This won't compile because `Position` can't be printed with `{:?}`.
// Add the attribute that derives `Debug` for it.
//
// Run with `rustlings run formatting2`, get a hint with `rustlings hint formatting2`.

// TODO: Add `#[derive(Debug)]` on the line above this struct.
struct Position {
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
    fn debug_format() {
        let p = Position { x: 1, y: 2 };
        assert_eq!(format!("{p:?}"), "Position { x: 1, y: 2 }");
    }

    #[test]
    fn pretty_debug_format() {
        let p = Position { x: 1, y: 2 };
        // `{:#?}` is the multi-line "pretty" form of `{:?}`.
        assert_eq!(format!("{p:#?}"), "Position {\n    x: 1,\n    y: 2,\n}");
    }
}
