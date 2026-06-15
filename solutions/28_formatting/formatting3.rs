// formatting3.rs
//
// Solution: implement `fmt::Display` by writing the desired text into the
// formatter with `write!`. This also provides `.to_string()` automatically.

use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
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
