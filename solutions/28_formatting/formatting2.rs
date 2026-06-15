// formatting2.rs
//
// Solution: deriving `Debug` lets the compiler generate the `{:?}` (and the
// pretty `{:#?}`) representation automatically.

#[derive(Debug)]
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
        assert_eq!(format!("{p:#?}"), "Position {\n    x: 1,\n    y: 2,\n}");
    }
}
