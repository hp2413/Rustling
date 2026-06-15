// traits8.rs
//
// Solution: implement `std::ops::Add` for `Point`, setting `Output = Point` and
// returning a new `Point` whose fields are the component-wise sums. With that,
// the `+` operator works on `Point` values.

use std::ops::Add;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
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
